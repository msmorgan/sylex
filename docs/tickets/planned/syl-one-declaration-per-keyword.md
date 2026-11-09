---
needs: [syl-life-totals-exchange-operand, syl-bodiless-keywords, syl-single-face-card-call]
---

# One keyword, one declaration

## Why

Several keywords are declared many times, once per printed variant: `exchange` six ways
(`exchangeCards`, `exchangeControl`, `exchangeLifeTotals`, `exchangeTextBoxes`, `exchangeValues`,
`exchangeZones`, beside `exchange` and `exchangeWith`), `hexproof` and `hexproofFrom`, and
`partner` with `partnerWith`, `partnerCharacterSelect`, `partnerFatherAndSon`,
`partnerFriendsForever`, `partnerSurvivors`, `chooseABackground` and `doctorsCompanion`: the
partner abilities are partner, partner—[text], partner with [name], choose a Background and
Doctor's companion [CR#702.124a], and `partner.syl` already lists the last two among its
variants. The `exchange` variants already share one Deed
label, `"exchange"`.

Two narrower tickets rewrite the same declarations first: `syl-life-totals-exchange-operand`
(`exchange.syl`, `exchangeLifeTotals.syl`, *Mirror Universe*) and `syl-bodiless-keywords` (the
seven partner-ability files).

This ticket lands after `syl-single-face-card-call`, which re-indents every single-faced canon card it edits.

## Design

- A keyword is one Definition, and its printed variants are Templates or parameters over it.

## Done when

- `exchange`, `hexproof` and `partner` are each one Definition; each variant is a Template or an
  argument over it, keeping its printed wording; every caller in `plugins/` is updated.
- Any variant kept as its own Definition states in its doc comment the rule that makes it one.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- The five `exile` keyword actions: merging them merges their Deeds, so it is
  `syl-one-exile-declaration`.
- Whether `lifeTotals` folds into `values`: `syl-life-totals-exchange-operand`.

## Pointers

`plugins/builtin/definitions/keyword_action/exchange*.syl`,
`plugins/builtin/definitions/keyword_ability/{hexproof,hexproofFrom}.syl`,
`plugins/builtin/definitions/keyword_ability/{partner*,chooseABackground,doctorsCompanion}.syl`,
`docs/decisions/semantics.md` §3.1.
