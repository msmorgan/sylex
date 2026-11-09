---
needs: []
---

# One keyword, one declaration

## Why

Several keywords are declared many times, once per printed variant: `exchange` six ways
(`exchangeCards`, `exchangeControl`, `exchangeLifeTotals`, `exchangeTextBoxes`, `exchangeValues`,
`exchangeZones`, beside `exchange` and `exchangeWith`), `hexproof` and `hexproofFrom`, and
`partner` with `partnerWith`, `partnerCharacterSelect`, `partnerFatherAndSon`,
`partnerFriendsForever` and `partnerSurvivors`. A keyword the rules define once
[CR#701.1,702.1] should be one Definition, its variants Templates or parameters over it.

## Done when

- `exchange`, `hexproof` and `partner` are each one Definition; each variant is a Template or an
  argument over it, keeping its printed wording; every caller in `plugins/` is updated.
- Any variant kept as its own Definition states in its doc comment the rule that makes it one.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- The five `exile` keyword actions: whether they share one Deed is an open question of
  `syl-explicit-deeds`.
- Whether `lifeTotals` folds into `values`: `syl-life-totals-exchange-operand`.

## Pointers

`plugins/builtin/definitions/keyword_action/exchange*.syl`,
`plugins/builtin/definitions/keyword_ability/{hexproof,hexproofFrom}.syl`,
`plugins/builtin/definitions/keyword_ability/partner*.syl`, `docs/decisions/semantics.md` §3.1.
