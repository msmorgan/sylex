---
needs: [syl-result-reads, syl-one-declaration-per-keyword, syl-ability-words-once, syl-this-head-template, syl-spelling-optional-segments, syl-one-deed-per-keyword-action, cleanup-step6-constructs-and-cards, cleanup-step6-rules-and-keywords, cleanup-step6-deeds, syl-number-snapshot]
---

# Delete `.syl` declarations nothing needs

## Why

`plugins/builtin/` carries declarations no card or rule needs: about 78 primitives with no user
and 63 used once (re-count when claimed), six `gift…` Templates (`giftACard`, `giftAFood`,
`giftATappedFish`, `giftATreasure`, `giftAnExtraTurn`, `giftAnOctopus`), and `manifestPlacement`
(`definitions/keyword_action/manifestPlacement.syl`), a placement-only fragment of manifest kept
for a bench that no longer exists.

Other tickets decide the fate of declarations this sweep would otherwise delete or fold, so they
land first: `syl-result-reads` (the coin and die reads with no user),
`syl-one-declaration-per-keyword` (the unused `partner` variants),
`syl-ability-words-once` (the unused `abilityWord` Template), `syl-this-head-template`
(`thisClass`, `thisEmblem` and `thisSpacecraft`, unused), `syl-spelling-optional-segments` (the
`enters…`, `activatedOnly…`, `prevent…` and `defines…` Templates it merges, several unused or used
once) and `syl-one-deed-per-keyword-action` (the unused `exchange` variants, and `exileFaceDownFrom`
and `exileWithCounters`, used once each).

It sweeps after the three middle stages (`cleanup-step6-constructs-and-cards`,
`cleanup-step6-rules-and-keywords`, `cleanup-step6-deeds`), which still add and remove
declarations.

## Open questions

- What happens to a declaration used once: folded into its user, kept, or something else.
- What happens to the six `gift…` Templates.

## Done when

- Every unused declaration is deleted, or kept with a doc comment naming the Vintage-legal card or
  rule that needs it; the landing lists both.
- Each declaration used once, and each of the six `gift…` Templates, is handled as settled; the
  landing lists each.
- `manifestPlacement` is gone, and nothing names it.
- Declaration order stays the printed order (`syl.md` §5).
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/`, `plugins/builtin/templates/ability/gift*.syl`,
`plugins/builtin/definitions/keyword_ability/gift.syl`,
`plugins/builtin/definitions/keyword_action/manifestPlacement.syl`.
