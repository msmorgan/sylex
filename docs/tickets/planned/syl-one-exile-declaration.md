---
needs: [syl-explicit-deeds]
---

# One declaration for exile

## Why

Exile is one keyword action [CR#701.13a] declared five times: `exile`, `exileFrom`,
`exileFaceDown`, `exileFaceDownFrom` and `exileWithCounters`
(`plugins/builtin/definitions/keyword_action/`). Each declares its own Deed under the label
`"exile"`, so "exiled this way" and "can't be exiled" can match them only by that label. Merging
the declarations merges their Deeds, which is `syl-explicit-deeds`' open question on one shared
exile Deed; this ticket builds the answer.

## Done when

- Exile is one Definition with one Deed, as `syl-explicit-deeds` settles; the face-down, origin
  and with-counters variants are Templates or arguments over it, keeping their printed wording,
  and every caller in `plugins/` is updated.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

The other keywords declared many times (`exchange`, `hexproof`, `partner`):
`syl-one-declaration-per-keyword`.

## Pointers

`plugins/builtin/definitions/keyword_action/exile*.syl`, `docs/decisions/semantics.md` §3 and §3.1.
