//! Spawning a child and capturing what it said. The gates use it, and a project's own commands
//! may too: a new command calls this and never edits it.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

/// What to spawn. `envs` are added to the inherited environment, never replacing it.
pub struct Spec<'a> {
    /// The program to run.
    pub program: &'a str,
    /// Its arguments.
    pub args: &'a [&'a str],
    /// Variables added to the inherited environment.
    pub envs: &'a [(&'a str, &'a str)],
    /// The child's working directory. Gates pass the project root, so the child and the
    /// log paths agree on where `target/` is whatever directory the tool was invoked from.
    pub cwd: &'a Path,
}

/// A finished child. `output` is stdout and stderr combined; the two streams are read
/// concurrently, so their interleaving is best-effort, and every byte of both is present.
pub struct Completed {
    /// Every byte of stdout and stderr, combined.
    pub output: Vec<u8>,
    /// Whether the child exited with success.
    pub success: bool,
}

/// Runs the child to completion, capturing both streams. When `live` is set, every chunk
/// is also written to this process's stdout as it arrives.
///
/// An `Err` is a failure to spawn or to read, not a nonzero child: that is
/// `Completed { success: false, .. }`, with whatever output the child produced.
pub fn run_captured(spec: &Spec, live: bool) -> std::io::Result<Completed> {
    let mut child = Command::new(spec.program)
        .args(spec.args)
        // Never inherited: the variable makes a rustc build nightly-equivalent and
        // libtest's unstable-flag check reads its mere presence, so a caller who exported
        // it would let a gate pass nightly-gated code that CI rejects. A spec that needs a
        // value sets it explicitly through `envs`.
        .env_remove("RUSTC_BOOTSTRAP")
        .envs(spec.envs.iter().copied())
        .current_dir(spec.cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // One shared buffer, one reader thread per stream. Reading both concurrently is what
    // prevents the deadlock where the child blocks writing the pipe nobody is draining.
    let sink: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
    let stdout = child.stdout.take().expect("stdout was piped above");
    let stderr = child.stderr.take().expect("stderr was piped above");
    let readers = [
        spawn_reader(stdout, Arc::clone(&sink), live),
        spawn_reader(stderr, Arc::clone(&sink), live),
    ];
    let status = child.wait()?;
    for reader in readers {
        reader.join().expect("a pipe reader panicked")?;
    }

    let output = Arc::try_unwrap(sink)
        .expect("both readers joined, so no clone outlives this")
        .into_inner()
        .expect("no reader holds the lock after joining");
    Ok(Completed {
        output,
        success: status.success(),
    })
}

fn spawn_reader(
    mut stream: impl Read + Send + 'static,
    sink: Arc<Mutex<Vec<u8>>>,
    live: bool,
) -> std::thread::JoinHandle<std::io::Result<()>> {
    std::thread::spawn(move || {
        let mut chunk = [0u8; 8192];
        loop {
            let n = stream.read(&mut chunk)?;
            if n == 0 {
                return Ok(());
            }
            if live {
                // Best-effort by design: a closed stdout must not kill the capture,
                // which the log still needs.
                let _ = std::io::stdout().write_all(&chunk[..n]);
            }
            sink.lock()
                .expect("a reader panicked while holding the lock")
                .extend_from_slice(&chunk[..n]);
        }
    })
}

/// stdout is best-effort: when the reader closed the pipe (`cargo x gates | head`), what
/// is printed truncates, and never the run, the logs or the exit code.
pub fn say(text: &str) {
    let _ = std::io::stdout().write_all(text.as_bytes());
}

/// stderr, same contract as `say`.
pub fn complain(line: &str) {
    let _ = writeln!(std::io::stderr(), "{line}");
}

/// A step name, printed before its child runs so a long silence is attributed. Terminal
/// only: in a capture the carriage return does not erase, and the captured report would
/// carry every announce beside its result line.
pub fn announce(name: &str) {
    use std::io::IsTerminal;
    if std::io::stdout().is_terminal() {
        say(&format!("{name:<10} ...\r"));
        let _ = std::io::stdout().flush();
    }
}

/// The project root: the nearest ancestor of `start` holding a file named `marker`, such as the
/// checker's manifest. `None` when no ancestor holds one.
pub fn project_root(start: &Path, marker: &str) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join(marker).is_file())
        .map(Path::to_path_buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    // The claim: both streams are captured, and a nonzero child is a completed run, not
    // an error. Mutation check: dropping the stderr reader loses "on-err" and the test
    // names it; mapping a nonzero status to Err fails the success assertions.
    #[test]
    fn captures_both_streams_and_the_exit_status() {
        let script = "echo on-out; echo on-err >&2; exit 3";
        let spec = Spec {
            program: "sh",
            args: &["-c", script],
            envs: &[],
            cwd: Path::new("."),
        };
        let done = run_captured(&spec, false).expect("sh spawns");
        let text = String::from_utf8(done.output).expect("echo emits UTF-8");
        assert!(text.contains("on-out"), "stdout was captured");
        assert!(text.contains("on-err"), "stderr was captured");
        assert!(!done.success);
    }

    #[test]
    fn a_passing_child_reports_success() {
        let spec = Spec {
            program: "true",
            args: &[],
            envs: &[],
            cwd: Path::new("."),
        };
        assert!(run_captured(&spec, false).expect("true spawns").success);
    }

    #[test]
    fn an_added_env_reaches_the_child() {
        let spec = Spec {
            program: "sh",
            args: &["-c", "printf %s \"$GATES_PROBE\""],
            envs: &[("GATES_PROBE", "reached")],
            cwd: Path::new("."),
        };
        let done = run_captured(&spec, false).expect("sh spawns");
        assert_eq!(done.output, b"reached");
    }

    #[test]
    fn a_missing_program_is_an_error_not_a_failed_gate() {
        let spec = Spec {
            program: "gates-no-such-program",
            args: &[],
            envs: &[],
            cwd: Path::new("."),
        };
        assert!(run_captured(&spec, false).is_err());
    }

    // The claim: the walk finds the nearest ancestor holding the marker, and nothing from the
    // filesystem root. Mutation check: returning `start` itself fails the marker assertion.
    #[test]
    fn project_root_walks_up_to_the_marker() {
        let here = std::env::current_dir().expect("the test has a working directory");
        let root = project_root(&here, "Cargo.lock").expect("the crate sits inside a workspace");
        assert!(root.join("Cargo.lock").is_file());
        assert!(here.starts_with(&root));
        assert_eq!(project_root(Path::new("/"), "Cargo.lock"), None);
    }
}
