---
needs: [cleanup-step6-spine]
---

# A number snapshot that surface rewrites diff against

## Why

`.syl` derives an antecedent's number from its determiner, quantity and coordination, and an
anaphor carries its own `Plurality` (`one`/`many`) on `pro`, `refl`, `theRest` and `the`. Nothing
checks that the two agree until reference resolution exists in step 7/8. The surface tickets
rewrite pronouns, determiners and coordination across the corpus, so a rewrite can silently
change an anaphor's number, or drop the `each` or `and` a later plural anaphor reads from, and
no gate would notice.

This is evidence, not a check: the script lives in `docs/evidence/` and is deleted once step 7/8
checks number.

## Design

- A one-off script in `docs/evidence/` expands Templates over `sylex_syl` trees and records, in
  reading order, each NounPhrase and DetPhrase's derived number (one, many or distributive) and
  each `pro`, `refl`, `theRest` and `the` call's reach, where it has one, and `Plurality`.
- Number is derived by these rules: `a`, `each` and `this` are singular; `allOf`, `bare` and
  `coord(and|andOr, …)` are plural; `target`, `count` and `slice` are singular only for a literal
  1; `headed` and similar heads inherit their number.
- Diffed against a baseline, it flags any change to an anaphor's `Plurality`, and any removal of
  `each`, `eachOf`, `doForEach` or `coord(and|andOr, …)` from an ability that contains a later
  `many` anaphor.
- The baseline is taken at the `cleanup-step6-spine` gate.

## Done when

- The script exists in `docs/evidence/`, and its output on the corpus at the spine gate is
  recorded there as the baseline.
- A diff of the baseline against itself flags nothing.

## Pointers

`plugins/builtin/primitives/pronoun/{pro,refl,theRest}.syl`,
`plugins/builtin/primitives/determiner/`, `plugins/builtin/primitives/noun/{coord,headed,slice}.syl`,
`crates/syl/`, `docs/decisions/semantics.md` §2.
