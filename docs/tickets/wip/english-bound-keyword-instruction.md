---
needs: []
---
# Retire unconditioned `KeywordInstruction` and align set-off keyword periods

## Why

`KeywordInstruction` (`Sentence -> KeywordLineItem '.' where head.KeywordForm = Parametric`) makes a
`Sentence` of any parametric keyword line item and a period, admitting both bound keywords with a
period (`*Forestwalk.`, `*Landwalk.`) and standalone non-em-dash keywords with a period (`*Equip
{2}.`) while adding a surplus `KeywordInstruction` derivation inside quoted granted keywords
(*Enchanted creature has "Cumulative upkeep {1}."*, Mana Chains; five units).
Meanwhile, in `crates/vocabulary/src/lib.rs`, `DashCondition` (*Companion — <Clause>.*, *To solve —
<Clause>.*, 23 units) omits its terminal `.` and `FinalParameter = SetOffCost` (relying on
`KeywordInstruction`), whereas periodless mana `DashCost` (*Suspend 1—{R}*, *Reinforce*, *Awaken*,
*Impending*) carries `FinalParameter = SetOffCost` despite ending in no period of its own.

## Design

- Retire unconditioned `KeywordInstruction`.
- In `crates/vocabulary/src/lib.rs`, emit a trailing `.` (`ParameterPart::Mark('.',
  Binding::Suffix)`) and `FinalParameter = SetOffCost` on `DashCondition` (matching verbal `Cost`
  in *Equip—Sacrifice a creature.*, read through `CompleteKeywordItem` -> `KeywordAbilityList` and
  `KeywordFollowedBySentences`), and drop `FinalParameter = SetOffCost` from periodless mana
  `DashCost` (*Suspend 1—{R}*).
- Admit a `.` separator before following `Sentences` on parametric non-`SetOffCost` keyword items
  (*Aeon Chronicler*'s `"Suspend X—{X}{3}{U}. X can't be 0."`, `"Enchant creature. …"`), and admit a
  terminal `.` on a standalone non-`SetOffCost` `KeywordLineItem` only when it carries a trailing
  supplement (*Firebending X, where X is … .*).

## Done when

- `KeywordInstruction` is retired, and neither `*Forestwalk.` nor standalone `*Equip {2}.` reads;
  real-card `!Sentence` assertions witness that no keyword line item with a period is a `Sentence`
  (Mana Chains, Aeon Chronicler, Ob Nixilis, the Adversary, Fire Lord Zuko's firebending line).
- All 23 `Companion` and `Case` (`To solve — … .`) units, verbal set-off cost units, multi-sentence
  keyword lines (*Aeon Chronicler*) and supplemented keyword lines (*Firebending X, where X is … .*)
  keep valid Readings, and the census shows no verdict loss on the supported corpus.
- `cargo xtask english strata --check`, `cargo test` and `cargo clippy --tests -- -D warnings` pass
  for `sylex_vocabulary` and `sylex_english`.

## Pointers

- `crates/english/grammar/english.fug`: `KeywordInstruction`, `CompleteKeywordItem`,
  `KeywordAbilityList`, `KeywordFollowedBySentences`, `KeywordForm`, `FinalParameter`.
- `crates/vocabulary/src/lib.rs` (`DashCost`, `DashCondition`, verbal `Cost`).
