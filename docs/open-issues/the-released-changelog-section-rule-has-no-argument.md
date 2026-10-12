---
kind: design
---
# The rule that a released changelog section's content never changes has no recorded argument

## Summary

`design@knowledge-architect@changelog-section-fixed-at-release` states "A released section's
content never changes; its structure may". Neither the head nor the commit that introduced the rule
gives a reason for it, so a later session cannot tell what premise would reopen it.

## Details

### What

The rule entered the record with the commit "The changelog records what a consumer must plan for,
and a release reviewer replaces the release-status axis". That commit's message is its
deliberation, by its own words ("its deliberation is this message"). It states the rule, "a
released section's content never changes, its structure may", and argues the rest of the change:
the entry tests, the review at the release, the lost alternatives. It gives no reason for the
released-section rule. The owner's words that the head quotes, "older changelog section allow
structural changes, and this passes as a structural change in my view", rule on one member, a skill
name written as its reference, and not on the rule.

Instances seen: one, pass 1 of the design-record axis of a project audit. Its check of the split of
`design@knowledge-architect@changelog-entries` found that the created head states no premise for
"A released section's content never changes": the only reason left in the text, "since the section
describes the release's net effect", argues the working section. The split was applied on the
owner's ruling, with this entry as the default for the missing argument.

Suspected mechanism, an assumption: the rule was taken as self-evident when the changelog policy
was argued, so nobody wrote its reason down. Candidate premises, none recorded: a consumer has
already planned a move against a released section, and a later correction would change that plan
unseen; a published crate ships its changelog, so a released section also exists in a published
package that cannot change.

### Why it matters

`primer@design-heads` asks a head for its standing argument, "every premise whose failure would
reopen the decision". `design@knowledge-architect@changelog-section-fixed-at-release` has none, so
a session cannot judge a proposal to correct a released section against a reason, and the rule
binds only by its presence. The rule is also what
`issue@knowledge-architect@a-mechanical-changelog-check` would assert mechanically, and a check
built on an unargued rule is costly to reverse.

### What would close it

The owner states the reason for the rule, or a search of the record finds one, and the head gains
that premise. Or a design discussion under `skill@knowledge-architect-design` argues the rule
against its nearest rival, a released section corrected after its release, and the head is
rewritten to the outcome. Re-entry point: the design discussion that
`issue@knowledge-architect@a-mechanical-changelog-check` asks for, which decides what a check
asserts about released sections, or earlier on the owner's word.
