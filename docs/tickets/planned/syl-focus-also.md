---
needs: [cleanup-step6-surface]
---

# Printed "also" has a written form

## Why

*Efflorescence* prints "Infusion — If you gained life this turn, that creature also gains trample
and indestructible until end of turn." Its canon file writes
`gain(that(creature), [trample, indestructible], untilEndOfTurn)`, and no declaration spells
"also", so the Reading's "also" matches no Construction and the card does not round-trip. It is
the only canon card that prints "also" in rules text today.

A scan of Vintage-legal Oracle rules text with reminder text removed (script and output in
`docs/evidence/syl-focus-also/`) finds "also" on 58 cards:

- **"also gains / has / gets / loses"** on an object another effect of the same spell or ability
  already changed (34 cards, 37 sentences): *Efflorescence*, *Temur Battle Rage* ("That creature
  also gains trample until end of turn if you control a creature with power 4 or greater"),
  *Jetmir, Nexus of Revels* ("Creatures you control also get +1/+0 and have trample as long as
  you control six or more creatures"), *Azure Beastbinder* ("it also has base power and toughness
  2/2"), *Goatnap* ("it also gets +3/+0").
- **"also" on a second effect of the same event or condition**, a different verb or patient (14
  cards): *Goblin Barrage* ("If this spell was kicked, it also deals 4 damage to target player or
  planeswalker"), *Liliana's Triumph* ("each opponent also discards a card"), *Finale of Glory*
  ("also create X 4/4 white Angel creature tokens"), *Geistlight Snare* ("It also costs {1} less
  to cast"), *Beast Mode* (sentence-initial "Also put a +1/+1 counter …").
- **"also" on a parallel event by another subject** (5 cards): *Okk* ("can't attack unless a
  creature with greater power also attacks"), *Scarred Puma*, *Ekundu Cyclops*, *Parnesse, the
  Subtle Brush* ("may also copy that spell"), *Sower of Discord* ("the other chosen player also
  loses that much life").
- **"is also a <subtype list>"** (4 cards): *Burakos, Party Leader*, *Stonework Packbeast*,
  *Tajuru Paragon*, *Veteran Adventurer* ("… is also a Cleric, Rogue, Warrior, and Wizard").
  The meaning, a type added beside the others, is the `adds` QualityOp that "in addition to its
  other types" writes (269 cards, no "also"; `isAlso` spells it in a copy exception, and
  `syl-become-replaces-types` owns the `become` forms). The "is also a" wording has no Template.
- **"is also goaded"** (1 card): *Hot Pursuit*.

No use among the 58 was found to change what the effect does: each sentence means the same without
"also". No existing declaration spells "also" (`isAlso`'s wording is "in addition to its other
types").

## Open questions

- Where does printed "also" live: a Template per verb that carries it (`alsoGain`, `alsoGet`,
  `alsoDeal`, …), a modifier on the grant or action, or something the English layer supplies?
  - The English layer cannot supply it. A Reading holds the word "also" (`english.md` §7's round
    trip renders it from the Reading), and what the English layer derives instead of spelling is
    the closed list in `spelling.md` §1: inflection and agreement, case, voice, Gaps.
  - A Template per verb multiplies with the verbs that take it (gain, get, have, lose, deal,
    discard, create, put, regenerate, copy, attack, block, cost: 13 in the corpus) and is the
    shape `spelling.md` §1 avoids for voice and number.
  - A modifier that is an argument of `gain` would be a flag stored beside the term, which
    `spelling.md` §4 rules out.
  - Recommendation: one Template per host sort, `also(x)`, whose body is `$x` and whose wording
    adjoins "also" inside the clause its argument spells, as `cant` adjoins "can't" (one-sided
    adjunction, `spelling.md` §2); it adds no meaning. The sorts it wraps come from the classes
    above (Action for the grants and second effects, the continuous effect for *Jetmir*, the
    event in *Okk*'s condition), and the "is also a" type line is its own Template beside the
    "in addition to its other types" forms.

## Done when

- The Open question is settled in `docs/decisions/spelling.md`, and the declarations it calls
  for exist.
- *Efflorescence* writes "also" in its canon file, and every canon card that prints "also" in
  rules text writes it; the scan's canon section lists each.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

Writing the 57 non-canon cards. The `become` forms of "in addition to its other types":
`syl-become-replaces-types`.

## Pointers

`docs/decisions/spelling.md` §1–4; `docs/decisions/english.md` §3, §7;
`plugins/canon/cards/Efflorescence.syl`; `plugins/builtin/templates/ability/isAlso.syl`;
`plugins/builtin/templates/action/become.syl`; `docs/evidence/syl-focus-also/`.
