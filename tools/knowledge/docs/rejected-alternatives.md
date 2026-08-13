# Rejected Alternatives

**Families named for the subjects that read them** — lost to `#families-are-the-checks`. `live`. A
family per review axis would let a reader ask for its own subject in one word instead of listing the
checks that serve it. It loses on `thaum@tools/CLAUDE.md`: nothing about this repository is compiled into the
tool, and every list a check reads comes from `thaum@knowledge.toml`. A subject's name inside
`Only::parse` is exactly that repository knowledge, compiled in. Which families serve a subject
belongs to whoever reads them.

**One family per invocation, instead of a set** — lost to `#families-are-the-checks`. `live`. It
needs no set type and no comma parsing, and each invocation stays one word. It loses to
`knowledge#model-then-checks`, which records what a walk costs: the walk happens once per invocation, so a
caller wanting five families reads every live document five times, which is the shape the single
walk was built to remove.

**A slug unique across the whole project, with the component named for the reader only** — lost to
`knowledge#a-slug-belongs-to-a-component`. `live`. It keeps one meaning per word everywhere and needs
no lookup to resolve a reference. It loses because it makes every component's vocabulary global: two
components cannot each decide something they call the same word, and the second one to want the word
has to take a worse one.

**A reference with no component read as one inside its own component** — lost to
`knowledge#a-slug-belongs-to-a-component`. `live`. It would leave a pointer inside a component as
short as it was before components existed, and qualify only the crossings. It loses on what a
reference has to carry by itself: the same text would name different decisions depending on which
file it sits in, so moving a document between components would silently retarget every unqualified
reference in it.

**A third-party mirror as the source** — lost to `knowledge#watch-reads-the-page`. `live`. Mirrors keep
stable index pages and would be less brittle than Wizards' HTML. It loses on what the corpus is: the
Comprehensive Rules are this project's only authority, and putting a third party between the project
and its authority as the _change signal_ is a dependency nothing else here has.
