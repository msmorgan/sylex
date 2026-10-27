---
needs: [plugins-v3-canon]
---
**[port-required: semantics-v3] Shrink the hand-written `lean/Semantics/Cards/`
bench to zero (`docs/decisions/semantics-v3.md` §4 item 10).** Every `Spelled`
card in `lean/Semantics/Cards/*.lean` moves into its own `.lean` file under
`plugins_v3/canon/cards/<name>.lean`, loaded by `load_bundle "../plugins_v3/canon"`
in `lean/Semantics/Cards.lean`, and the `lean/Semantics/Cards/*.lean` submodules
are deleted. Phrase-level construction pins and theorems in `lean/Semantics/Cards/*.lean`
that are theorems about constructions rather than `Spelled` cards move into
`lean/Semantics/Proofs/` so no test coverage is lost. Standard constraints apply.
