---
needs: []
---
# American spelling throughout the repo

Standard constraints apply. The project implements an American game for an American owner, so the
repo is written in American English. British spellings came in during the english-v4 work and are
now in docs, doc comments, tickets and some identifiers (about 150 occurrences in 40 files, counted
2026-11-03): behaviour, colour, honour, modelled, labelled, labelling, cancelled, recognise,
summarise, normalising, licence, and others.

## Change

- Respell docs, tickets, doc comments, comments and test names: behavior, color, honor, modeled,
  labeled, labeling, canceled, recognize, summarize, normalize, license, catalog, center, and the
  like.
- Rename identifiers that carry a British spelling (for example `LabelledAbility`,
  `LabelledModalHeader`, `LabelledCostedMode`), with their call sites, `.syl` uses and glossary
  entries, as one rename each.
- Add one line to `CLAUDE.md` saying the repo uses American spelling, and fix its own
  "behaviour".
- Keep verbatim quotations as written: CGEL excerpts, rules text and other quoted sources. Leave
  `data/`, generated files and archived experiments alone.
- Not every hit is British: "analyses" (plural noun) and "initialisms" are American too. Check each
  word before changing it.

## Tests

- Every crate whose identifiers change builds, with its tests and clippy clean; `cargo xtask lean
  check` if a Lean or `.syl` name changes; `cargo xtask cite check` 0 stale.
- A grep for the respelled words finds only verbatim quotations.
