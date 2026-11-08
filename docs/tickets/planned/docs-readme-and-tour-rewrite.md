---
needs: [ci-lean-gate, ci-noncanon-gate, comment-discipline-sweep, engine-verb-ident-table, readme-oracle-text-primer, readme-reviewer-path, todo-harness-ci]
---

# Rewrite the README and guided tour, and verify them on a fresh clone

**Why.** `README.md` and `docs/guided_tour.md` are the project's public face, and both describe a
project that no longer exists: the Lean workbench as active, the v1 pipeline as the production
path, and a chart-parser framing that overstates what parses. A reviewer who follows them hits
dead ends. They need rewriting together, then checking the way a stranger would use them.

**Done when.**
- The owner has agreed the outline of both before drafting, and approves the draft.
- The README carries the vision and rationale from `docs/decisions/architecture.md`, describes
  the English grammar, and says nothing that goes stale with the next landing; the tour follows one
  current `.syl` card through the pipeline.
- Every path and link in both resolves.
- A disposable fresh clone, with no pre-existing `data/`, `target/`, mtg-rules skill or
  repository-specific environment variables, follows the README literally (build, first run,
  `cargo test`, every documented gate) and succeeds. The landing reports the commands,
  toolchain, OS and result.
- Every externally visible README claim is checked against that run, and `ci.yml` comments about
  gaps that have closed are gone.
- `kata kanban check`, the citation gates and the noncanon fixed-seed suite pass.

**Not in scope.** `comment-discipline-sweep`, `core-player-action-merge-leftovers`, and helper
extraction.

**Pointers.** `README.md`, `docs/guided_tour.md`, `docs/decisions/architecture.md`,
`.github/workflows/ci.yml`, `readme-oracle-text-primer`.
