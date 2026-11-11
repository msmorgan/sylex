---
needs: [cleanup-step6-spine, syl-spelling-optional-segments, syl-nominal-descriptions, syl-continuous-effect-subject]
---

# Characteristic-setting is written as printed, and plain `become` replaces types

## Why

`plugins/builtin/templates/action/become.syl`'s comment spells it "<subject> becomes <added> in
addition to its other types", and its body hardcodes `qualityChange(…, adds, …)`, so the bare word
`become` keeps the old types. The rules default the other way: an effect that sets an object's
card type or subtypes replaces the existing ones [CR#205.1a]. Keeping them is the marked case,
printed as "in addition to its other types", "still a [type]", "becomes an artifact creature", or
"[creature type] artifact creature", which keeps all but the creature types [CR#205.1b].

All four callers print a retaining form:

- *Answered Prayers*: "it becomes a 3/3 Angel creature with flying in addition to its other types
  until end of turn";
- *Dread Slaver*: "That creature is a black Zombie in addition to its other colors and types";
- *Infernal Vessel*: "It's a Demon in addition to its other types";
- `keyword_action/amass.syl`: "it becomes a [subtype] in addition to its other types"
  [CR#701.47a].

`QualityOp` has `sets` (`primitives/qualityOp/sets.syl`), as `becomeColor` already uses.

What an object is set to is written as a raw record with no wording: the three `become` calls
above and `amass.syl`'s, 30 `bundle(…)` payloads of `qualityChange` in 27 canon files (*Blood
Moon*'s "Nonbasic lands are Mountains", *Frogify*, *Gideon, Ally of Zendikar*, *Arcum's
Weathervane*, …), and *Yedora, Grave Gardener*'s `faceDownAs` ("It's a Forest land"). A record
cannot say what the cards print apart: "becomes a 3/3 Angel creature" against "with base power and
toughness 3/3" (*Gift of Tusks*), "that's still a planeswalker" (*Gideon, Ally of Zendikar*), "is
no longer snow" (*Arcum's Weathervane*). *Frogify* ("Enchanted creature loses all abilities and is
a blue Frog creature with base power and toughness 1/1") writes `it` as the subject of two of its
three parts, where the card prints one subject and no "it".

## Design

- Plain `become` writes `sets`: the given types replace the existing ones [CR#205.1a].
- Each retaining form is its own Template named for its printed wording, writing `adds` as that
  wording says [CR#205.1b], so the meaning lives in which Template a card calls
  (`syl-spelling-conventions`).

## Open questions

- **How what an object becomes is written.** Options: (a) as the copula's complement, a
  `Predicative` built from `syl-nominal-descriptions`'s sorts ("a 3/3 Angel creature with
  flying", "a Forest land", "snow"), lowered to the characteristics it sets; (b) a fixed-slot
  Template per shape of record, as `creatureToken` is for tokens. Recommendation: (a), which
  `syl-token-records-to-templates` asks the same of for created tokens; the two answers are one.
- **The subject.** `become`, `qualityChange` and `becomeColor` take their subject first; with
  `syl-continuous-effect-subject` they take none. Recommendation: write these calls against
  that ticket's declarations, so *Frogify*'s one printed subject is written once.

## Done when

- The open questions are answered.
- No raw `{characteristics = …}` record remains outside a token slot under `plugins/`; every
  caller is checked against its Oracle text and calls the form it prints, and the landing lists
  each with its printed wording.
- No Template whose name says "become" keeps the old types unless its name says so.
- *Gift of Tusks* ("Until end of turn, target creature loses all abilities and becomes a green
  Elephant with base power and toughness 3/3.") is written in `plugins/canon/` with plain
  `become`, the replacing form's first caller.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass;
  `cargo xtask cite check --list-noncompliant` is empty and `cargo xtask cite check` reports
  0 stale, after `cargo xtask cite bless` for newly cited rules.

## Not in scope

- Records in a token slot: `syl-token-records-to-templates`.
- Copy exceptions ("except it has haste"): `syl-copy-exceptions`.

## Pointers

`plugins/builtin/templates/action/{become,becomeColor}.syl`,
`plugins/builtin/templates/ability/qualityChange.syl`, `plugins/builtin/primitives/qualityOp/`,
`plugins/builtin/primitives/static/bundle.syl`, `plugins/builtin/primitives/zone/faceDownAs.syl`,
`plugins/builtin/definitions/keyword_action/amass.syl`,
`plugins/canon/cards/{Answered Prayers,Dread Slaver,Infernal Vessel,Frogify,Yedora, Grave
Gardener}.syl`; `docs/evidence/syl-token-records-to-templates/scan.out`.
