---
needs: []
---

# Bring the README up to date as the project's landing page

**Why.** The root `README.md` is the project's public landing page, linked from the owner's
resume. It describes the architectural vision and the rationale behind it, and avoids metrics that go
stale, so that it doesn't need constant upkeep. It has not been updated since much of the project
was built and iterated on, most visibly the English grammar: a lexicalist feature-unification grammar
parsed by an Earley SPPF chart parser, which reads every in-scope card except three Reserved List
cards that are declined for stated reasons (`crates/english/declined.txt`).

**Done when.**
- The README presents the vision and rationale as it does now, updated where the design has moved
  on, and describes what has been built at a level that does not go stale.
- Every path, command and link in it resolves.
- The owner has read and approved the draft. This is a design ticket: the outline is agreed with
  the owner before drafting.

**Not in scope.** Changing the design documents the README summarizes.

**Pointers.** `README.md`, `docs/decisions/architecture.md` (the vision the README summarizes for
a human reader), `docs/decisions/semantics.md`, `docs/decisions/syl.md`, `docs/decisions/english.md`,
`docs/decisions/english-lexical-analysis.md`, `crates/english/declined.txt`.
