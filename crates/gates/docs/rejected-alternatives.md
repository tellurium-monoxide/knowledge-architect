# Gates — rejected alternatives

The alternatives that lost to a decision of `path@gates@docs/design.md`, each with what it lost to
and why.

## Capturing a child's two streams through one regular file instead of two pipes

Lost to `design@gates@one-spawn-helper`. `live`. Its one reason was a child that
refuses a pipe on stdout, which the checker's `check` did for a while, per the entry on that
refusal in `path@core@docs/rejected-alternatives.md`; no child refuses one now. Its one gain
is that both streams keep the order the child wrote them in. It was built, with the file in the
system temporary directory, opened in append mode for both streams and read back by polling, and
an adversarial review reproduced three losses the pipes do not have:

- a child that reopens its standard output by the device path, /dev/stdout, truncates the capture:
  `sh -c 'echo first-line-that-is-long; echo b > /dev/stdout; echo c'` captured `b` and `c` alone;
- output a descendant writes after the child exits is dropped:
  `sh -c '(sleep 0.3; echo late) & echo early'` captured `early` alone;
- a killed run leaves the file behind, and since the name was the process id and a counter,
  created with `create_new`, a later run that reuses the process id fails every gate.

No gate child did any of the three, so the losses are latent. They are what a reintroduction
has to answer.
