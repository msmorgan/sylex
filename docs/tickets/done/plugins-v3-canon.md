---
needs: [semantics-v3-dsl-and-plugins, plugins-v3-builtin-primitives-and-templates]
---
**Extract the hand-authored exemplar card bench from `lean/Semantics/Cards/*.lean`
into individual `.lean` files in `plugins_v3/canon/cards/`, loaded by
`load_bundle "../plugins_v3/canon"`, and retire the hand-written
`lean/Semantics/Cards/` modules (`lean-hand-bench-retirement`)
(updated per [`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md) §4 item 10).**
Every card in `plugins_v3/canon/cards/<name>.lean` contains a single `card`
declaration (`card <name> : Spelled := { ... }`) checked directly by `lake build`
(`cargo xtask lean check`) and read by `deckmaste_semantics_v3`. Standard
constraints apply.
