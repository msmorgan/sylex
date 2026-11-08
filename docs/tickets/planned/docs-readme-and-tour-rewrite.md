---
needs: [ci-lean-gate, ci-noncanon-gate, comment-discipline-sweep, engine-verb-ident-table, readme-reviewer-path, todo-harness-ci]
---

# Rewrite the README and guided tour, and verify them on a fresh clone

## Why

`README.md` and `docs/guided_tour.md` are the project's public face, and both describe a
project that no longer exists: the Lean workbench as active, the v1 pipeline as the production
path, and a chart-parser framing that overstates what parses. A reviewer who follows them hits
dead ends. They need rewriting together, then checking the way a stranger would use them.

## Done when

- The owner has agreed the outline of both before drafting, and approves the draft.
- The README carries the vision and rationale from `docs/decisions/architecture.md`, describes
  the English grammar, and says nothing that goes stale with the next landing; the tour follows one
  current `.syl` card through the pipeline.
- Near the top, ahead of Design, the README says what Oracle text is, so a reader who does not
  play Magic learns why card text is uniform enough to compile: Wizards of the Coast's
  authoritative rules text for each card, maintained centrally rather than fixed at print time,
  so an old card carries current wording; and written to a house template, the same concept in
  the same words in the same order across the card pool, with the existing corpus reissued when
  the template changes. It calls Oracle text "templated", "regimented" or a "controlled
  sublanguage", never a "regular language" (formally it is not one, and "regular" elsewhere in
  the repo means regular morphology). It cites `docs/oracle-style-guide.md`, if at all, as the
  project's own reconstruction from the CR and the corpus, never as a published standard, and it
  describes the input corpus, not the parser. The GitHub repository's About blurb carries the
  same framing in one line.
- Every path and link in both resolves.
- A disposable fresh clone, with no pre-existing `data/`, `target/`, mtg-rules skill or
  repository-specific environment variables, follows the README literally (build, first run,
  `cargo test`, every documented gate) and succeeds. The landing reports the commands,
  toolchain, OS and result.
- Every externally visible README claim is checked against that run, and `ci.yml` comments about
  gaps that have closed are gone.
- `kata kanban check`, the citation gates and the noncanon fixed-seed suite pass.

## Not in scope

`comment-discipline-sweep`, `core-player-action-merge-leftovers`, and helper
extraction.

## Pointers

`README.md`, `docs/guided_tour.md`, `docs/decisions/architecture.md`,
`.github/workflows/ci.yml`, `docs/oracle-style-guide.md`.
