---
needs: [cleanup-step6-surface]
---

# A characteristic axis has one word

## Why

Two sorts name the characteristic a group is formed by or compared on: `KindAxis` (`cardType`,
`color`, `counterKind`, `name`, `subtype(host, scope)`, `value(stat)`), read by `doForEachKind`,
`eachKind` and `distinctCount`, and `QualitySort` (`cardName`, `color`, `counterKind`, `number`,
`subtype(host)`), read by `shares`, `agree`, `ofChosen`, `quality` and the `entersChoosing`
Templates. One concept has two words across them: a card's name is `name` at `KindAxis` and
`cardName` at `QualitySort`.

`doForEachKind(axis, domain, sort, body)` writes its axis twice, once at each sort, and every call
writes the same characteristic in both: `counterKind` and `counterKind` (`doubleCounters`, *Star
Pupil*, *Denry Klin, Editor in Chief*), `color` and `color` (*Rogues' Gallery*), `value(power)`
and `number` (*Celestial Judgment*), `name` and `cardName` (`legendRule`). The text prints the
characteristic once ("for each kind of counter", "for each color").

Subtype families are modeled several ways besides: `SubtypeSpace` (`creature`, `basicLand`) for
"every creature type" and "every basic land type", and `KindAxis.subtype(host, scope)`, whose
required `scope` has one value, `basicOnly`, so a `KindAxis` subtype is always a basic land type
(*Tribal Flames*'s `subtype(land, basicOnly)`).

## Open questions

- Whether `KindAxis` and `QualitySort` are one sort, or which of their words each keeps.
- How a subtype family is named once across `SubtypeSpace`, `KindAxis.subtype` and
  `QualitySort.subtype`.

## Done when

- `doForEachKind` takes its characteristic once, and every call is updated.
- No characteristic has two words: a card's name is one word wherever an axis or sort names it.
- No parameter has a sort with a single value.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/builtin/primitives/{kindAxis,qualitySort,subtypeSpace,subtypeScope}/`,
`plugins/builtin/primitives/action/doForEachKind.syl`, `plugins/builtin/rules/sba/legendRule.syl`,
`plugins/builtin/definitions/keyword_action/doubleCounters.syl`.
