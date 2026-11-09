---
needs: [syl-result-reads, syl-one-declaration-per-keyword]
---

# Delete `.syl` declarations nothing needs

## Why

`plugins/builtin/` carries declarations no card or rule needs: about 78 primitives with no user
and 63 used once (re-count when claimed), six `gift…` Templates (`giftACard`, `giftAFood`,
`giftATappedFish`, `giftATreasure`, `giftAnExtraTurn`, `giftAnOctopus`) where one parameterized
form would do, and `manifestPlacement` (`definitions/keyword_action/manifestPlacement.syl`), a
placement-only fragment of manifest kept for a bench that no longer exists.

`syl-result-reads` decides the fate of the coin and die reads with no user, and
`syl-one-declaration-per-keyword` that of the unused `partner` and `exchange` variants, so both
land first.

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
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/`, `plugins/builtin/templates/ability/gift*.syl`,
`plugins/builtin/definitions/keyword_ability/gift.syl`,
`plugins/builtin/definitions/keyword_action/manifestPlacement.syl`.
