//! Every slug anchor referenced somewhere is defined exactly once.
//!
//! A slug is DEFINED where it opens a decision and REFERENCED anywhere else. A reference with
//! no definition is a pointer into nothing, which the head-and-commit split makes cheap to
//! create and expensive to notice; two definitions mean a rename left one behind, and the
//! reader who finds the stale one acts on it.

use std::collections::BTreeMap;

use crate::finding::Finding;
use crate::model::Model;
use crate::scan::Observation;

pub fn check(model: &Model) -> (Vec<Finding>, (usize, usize)) {
    let mut defined: BTreeMap<&str, Vec<(String, u32)>> = BTreeMap::new();
    let mut referenced: BTreeMap<&str, Vec<(String, u32)>> = BTreeMap::new();
    for doc in model.documents() {
        let name = doc.rel.display().to_string();
        for l in &doc.observations {
            match &l.what {
                Observation::SlugDef(s) => defined
                    .entry(s.as_str())
                    .or_default()
                    .push((name.clone(), l.line)),
                Observation::SlugRef(s) => referenced
                    .entry(s.as_str())
                    .or_default()
                    .push((name.clone(), l.line)),
                _ => {}
            }
        }
    }

    let mut out = Vec::new();
    for (slug, where_) in &referenced {
        if !defined.contains_key(slug) {
            let (file, line) = &where_[0];
            out.push(Finding::at(
                file,
                *line,
                format!("`#{slug}` is referenced but never defined"),
                "define it where the decision is made, or point at the slug that exists",
            ));
        }
    }
    for (slug, where_) in &defined {
        if where_.len() > 1 {
            let all: Vec<String> = where_.iter().map(|(f, l)| format!("{f}:{l}")).collect();
            let (file, line) = &where_[0];
            out.push(Finding::at(
                file,
                *line,
                format!(
                    "`#{slug}` is defined {} times: {}",
                    where_.len(),
                    all.join(", ")
                ),
                "delete all but one; a reference must resolve to exactly one anchor",
            ));
        }
    }
    (out, (defined.len(), referenced.len()))
}
