---
needs: [core-copy-grammar, engine-face-down]
---
**Layer 1 (copy and face-down) and layer 3 (text-changing) apply in the layer pass.**

## Why

`base_values` reads the printed face unconditionally. The copy inputs exist in core
(`EnterRider::AsCopy`, `StaticSpec::BecomesCopy`, both carrying a `CopySpec`) and
`crates/engine/src/copy.rs` already derives copiable values and folds `CopySpec` exceptions, but
nothing feeds them into layer 1a [CR#613.2a,707.2]: an `AsCopy` rider fizzles at runtime, so
Clone ("You may have this creature enter as a copy of any creature on the battlefield") enters
as a 0/0. Face-down characteristics (layer 1b, [CR#613.2b,708.2]) have no input until
`engine-face-down` lands. Layer 3's `Modification::SetText(String)` is a no-op arm, and a string
cannot carry the term rewrites the `.syl` `textChange` primitive names [CR#612.1,613.1c].

## Done when

- `base_values` derives an object's copiable values from its gathered `AsCopy`/`BecomesCopy`
  input, so after layer 1 the characteristics are the copiable values [CR#613.2c].
- An object entering "as a copy" is a copy as it enters, and the copied text's enters-replacements
  and enters triggers apply [CR#707.5]. Tests with Clone: it enters as a copy of a chosen
  creature, or as itself (0/0) when its controller declines.
- A face-down permanent derives the characteristics [CR#708.2a] lists, as its copiable values.
- Core carries a text change as a term rewrite in place of `SetText(String)`, and a layer-3
  text-changing effect rewrites the affected object's text (test with Sleight of Mind, Mind Bend
  or Artificial Evolution).

## Pointers

- `crates/engine/src/layer.rs`: `base_values` (its layer-1 seam doc comment), `layer_of`,
  the `SetText` arm of `apply_static`.
- `crates/engine/src/copy.rs`: copiable values, `apply_exceptions`, `has_unbuilt_enter_rider`.
- `crates/engine/src/resolve/action.rs`, `crates/engine/src/resolve/player_action.rs`: where
  `AsCopy` riders fizzle.
- `plugins/canon/cards/Clone.syl`; `plugins/builtin/primitives/static/textChange.syl`.
