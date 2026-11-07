---
needs: [ci-cite-gate, engine-todo-triage]
---

# Comment discipline: the reviewer-facing engine slice

## Why

The comments a cold reader of the engine meets first fail the comment rule worst: about 70
unresolved `Task N`/`P0.Wn` references across `crates/` (mostly `sylex_engine`), the
reviewer-directed essay on spell-copy cleanup in `sba.rs`, and the heaviest-commented engine
files. This slice fixes those without waiting for the whole `comment-discipline-sweep`.

## Done when

- `rg -n -e '\bTask [0-9]' -e 'P0\.W[0-9]' crates` returns no hits; each former hit is the
  durable invariant or a labeled `TODO`/`SEAM` naming a real ticket slug.
- `crates/engine/src/sba.rs`, `layer.rs` and `resolve/effect.rs`, plus the two other engine files
  with the most comment lines at claim time, have had a whole-file comment pass under the comment
  rule in `comment-discipline-sweep`'s Design.
  CR citations and invariants are kept.
- The diff changes no code outside comments.

## Pointers

- `comment-discipline-sweep` (the comment rule)
- `crates/engine/src/sba.rs` (`sweep`), `crates/engine/src/layer.rs`,
  `crates/engine/src/resolve/effect.rs`
