---
needs: [cleanup-step6-surface]
---

# *Engineered Explosives* names its sunburst

## Why

*Engineered Explosives* prints the keyword "Sunburst", but
`plugins/canon/cards/Engineered Explosives.syl` writes a charge-counter entry in its place:
`static(entersWithCounters(thisArtifact, colorsSpentToCast(thisArtifact), chargeCounter))`, one
branch of what sunburst means [CR#702.44a]. The keyword is declared
(`plugins/builtin/definitions/keyword_ability/sunburst.syl`), so the card can name it, and the
written-out copy carries none of the keyword's identity: nothing that asks whether a permanent has
sunburst sees it.

## Done when

- *Engineered Explosives*' text names the `sunburst` keyword where it prints "Sunburst", and its
  file comment no longer says the keyword is written out.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass;
  `cargo xtask cite check --list-noncompliant` is empty and `cargo xtask cite check` reports
  0 stale.

## Pointers

`plugins/canon/cards/Engineered Explosives.syl`,
`plugins/builtin/definitions/keyword_ability/sunburst.syl`.
