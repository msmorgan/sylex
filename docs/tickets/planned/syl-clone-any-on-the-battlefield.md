---
needs: [cleanup-step6-surface]
---

# "any creature on the battlefield" in a copy choice

## Why

*Clone* prints "You may have this creature enter as a copy of any creature on the battlefield."
and is written `mayHaveEnter(thisCreature, [asCopyOf(a(isCreature), [])])`. `a(isCreature)` spells
"a creature": it drops "any" and "on the battlefield", so the term recovers a different wording,
and `isCreature` holds of a creature card in any zone, so nothing in the term keeps the copied
object on the battlefield. No Template carries "any … on the battlefield".

A scan of Vintage-legal Oracle text for "any … on the battlefield" and "copy of …"
(`docs/evidence/syl-clone-any-on-the-battlefield/scan.py`) finds three canon cards with this
lost wording, all through `asCopyOf(a(…), …)`:

| Card | Oracle text | Written |
|---|---|---|
| *Clone* | "enter as a copy of any creature on the battlefield" | `asCopyOf(a(isCreature), [])` |
| *Quicksilver Gargantuan* | "enter as a copy of any creature on the battlefield, except it's 7/7" | `asCopyOf(a(isCreature), [hasPT(7, 7)])` |
| *Sculpting Steel* | "enter as a copy of any artifact on the battlefield" | `asCopyOf(a(isArtifact), [])` |

*Bruna, Light of Alabaster* ("any number of Auras on the battlefield") keeps its wording through
`counted(anyNumber, and([hasSubtype(aura), in(battlefield)]))`. No keyword body prints the
wording: the copy-making keywords (embalm, encore, eternalize, myriad, offspring, squad,
populate) copy "this" object or "a creature token you control". Outside canon, 44 Vintage-legal
cards print "copy of any …": 28 of them "any creature on the battlefield" (some with more after
it), 4 "any creature card in a graveyard" and 1 "any land card in a graveyard".

## Open questions

- Which word carries the wording. (a) A determiner Template "any <p>" (a choice that is not a
  target, as `a(p)` is today), with the location as a Predicate "on the battlefield" inside
  `p`, so the graveyard forms reuse the same "any"; `in(loc)` spells "in <loc>", so "on the
  battlefield" needs its own spelling. (b) One Template "any <p> on the battlefield".
  Recommendation: (a), since "any creature card in a graveyard" needs the same determiner with
  another location.

## Done when

- *Clone*, *Quicksilver Gargantuan* and *Sculpting Steel* each call words whose spelling is their
  printed "any … on the battlefield", and the term restricts the copied object to the
  battlefield.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/canon/cards/{Clone,Quicksilver Gargantuan,Sculpting Steel}.syl`,
`plugins/builtin/templates/ability/mayHaveEnter.syl`,
`plugins/builtin/primitives/zone/asCopyOf.syl`, `plugins/builtin/templates/determiner/a.syl`,
`plugins/builtin/primitives/predicate/in.syl`, `plugins/builtin/templates/predicate/isPermanent.syl`.
