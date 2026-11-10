---
needs: [cleanup-step6-surface]
---

# Banding is one keyword label with an optional quality

## Why

"Bands with other" is a special form of banding, and an effect that makes a permanent lose
banding takes all its "bands with other" abilities too [CR#702.22b]. Today the two are separate
labels, `banding` and `bandsWithOther` (`plugins/builtin/primitives/keywordLabel/`), over one
Ability-side primitive, `primitiveKeyword(keyword, quality)`. Nothing in the data ties
`bandsWithOther` to `banding`, so [CR#702.22b] would have to be special-cased. Two labels are
forced by today's `KeywordFamily` (fields `word` and `sort` only), which cannot pick out only the
instances that carry a quality, not by the rules.

## Design

- One label, `banding`. "Bands with other <quality>" is `banding` with a quality, through the
  generic wrapper's existing optional `quality`; the `bandsWithOther` label primitive goes, and
  the `bandsWithOther` Template wraps `primitiveKeyword(banding, quality = …)`.
- Losing banding takes the "bands with other" abilities because they are instances of the
  family [CR#702.22b], as losing hexproof takes every "hexproof from" [CR#702.11e].
- `KeywordFamily` gains a selector for the forms of a keyword that carry a quality, so *Shelkin
  Brownie* ("{T}: Target creature loses all "bands with other" abilities until end of turn.")
  is written over `banding`'s family restricted to those forms.
- *Tolaria* ("{T}: Target creature loses banding and all "bands with other" abilities until end
  of turn. Activate only during any upkeep step.") keeps both printed items: banding, and the
  family's forms with a quality.
- The optional `quality` stays on the generic `primitiveKeyword` wrapper, not on a
  banding-specific shape. Lowering refuses a quality on a keyword that takes none; the same slot
  can carry trample over planeswalkers [CR#702.19c].

## Open questions

- The selector's shape. A creature with "bands with other" counts as one without banding when
  bands are declared [CR#702.22c], and the damage-assignment rules name "a creature with
  banding" and the "bands with other [quality]" case separately [CR#702.22j,702.22k]; so the
  rules rows need the bare form as well, unlike hexproof, where any form counts as hexproof
  [CR#702.11e]. Options: (a) one `KeywordFamily` field with two words, the form without a
  quality and the forms with one, used by *Shelkin Brownie* and *Tolaria* (with a quality) and by
  the banding rules (without); (b) only a with-a-quality selector, the bare form read as the
  plain label. Recommendation: (a), since the banding rules need both sides by name.

## Done when

- `banding` is the only banding label; no `bandsWithOther` label primitive remains.
- *Shelkin Brownie* and *Tolaria* are written with their printed wording and resolve.
- [CR#702.22b] needs no special case: losing `banding` selects every instance of the family.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/primitives/keywordLabel/{banding,bandsWithOther}.syl`,
`plugins/builtin/templates/ability/{banding,bandsWithOther}.syl`,
`plugins/builtin/primitives/ability/primitiveKeyword.syl`,
`plugins/builtin/primitives/keywordTerm/anyIn.syl`, `KeywordFamily` in
`crates/xtask/src/dsl_check/schema.rs`, `plugins/canon/cards/{Shelkin Brownie,Tolaria}.syl`.
