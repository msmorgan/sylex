---
needs: [cleanup-step6-spine, syl-spelling-optional-segments]
---

# Plain `become` replaces types; the retaining forms are named for their wording

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

Written against the spine (`cleanup-step6-spine`). `syl-spelling-optional-segments` writes
`become.syl`'s duration `TODO(spelling)`, which this ticket also rewrites, and lands first.

## Design

- Plain `become` writes `sets`: the given types replace the existing ones [CR#205.1a].
- Each retaining form is its own Template named for its printed wording, writing `adds` as that
  wording says [CR#205.1b], so the meaning lives in which Template a card calls
  (`syl-spelling-conventions`).

## Done when

- Every caller is checked against its Oracle text and calls the form it prints; the landing lists
  each caller with its printed wording.
- No Template whose name says "become" keeps the old types unless its name says so.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass;
  `cargo xtask cite check --list-noncompliant` is empty and `cargo xtask cite check` reports
  0 stale, after `cargo xtask cite bless` for newly cited rules.

## Pointers

`plugins/builtin/templates/action/{become,becomeColor}.syl`,
`plugins/builtin/templates/ability/qualityChange.syl`, `plugins/builtin/primitives/qualityOp/`,
`plugins/builtin/definitions/keyword_action/amass.syl`,
`plugins/canon/cards/{Answered Prayers,Dread Slaver,Infernal Vessel}.syl`.
