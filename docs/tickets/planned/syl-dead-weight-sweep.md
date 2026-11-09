---
needs: []
---

# Delete `.syl` declarations nothing needs

## Why

`plugins/builtin/` carries declarations no card or rule needs: about 78 primitives with no user
and 63 used once (re-count when claimed), six `gift…` Templates (`giftACard`, `giftAFood`,
`giftATappedFish`, `giftATreasure`, `giftAnExtraTurn`, `giftAnOctopus`) where one parameterized
form would do, and `manifestPlacement` (`definitions/keyword_action/manifestPlacement.syl`), a
placement-only fragment of manifest kept for a bench that no longer exists.

## Done when

- Every unused declaration is deleted, or kept with a doc comment naming the Vintage-legal card or
  rule that needs it; the landing lists both.
- Each declaration used once is folded into its user or kept with the reason; the landing lists
  each.
- The six `gift…` Templates are one form over gift's argument, with every caller updated.
- `manifestPlacement` is gone, and nothing names it.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/`, `plugins/builtin/templates/ability/gift*.syl`,
`plugins/builtin/definitions/keyword_ability/gift.syl`,
`plugins/builtin/definitions/keyword_action/manifestPlacement.syl`.
