---
needs: [cleanup-step6-surface]
---

# Printed "also" has a written form

## Why

*Efflorescence* prints "Infusion — If you gained life this turn, that creature also gains trample
and indestructible until end of turn." Its canon file writes
`gain(that(creature), [trample, indestructible], duration = untilEndOfTurn)`, and no declaration
spells "also", so the Reading's "also" matches no Construction and the card does not
round-trip. It is the only canon card that prints "also" in rules text today.

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

Written against the surface (`cleanup-step6-surface`), which includes
`syl-continuous-effect-subject`; that ticket's subjectless phrase is what "also" wraps on a
continuous effect, and the gate already orders the two, so this ticket names no edge of its own.

## Design

- The English grammar attaches a central "also" to the verb phrase below the subject
  (`CentralAdverbial: VerbPhrase -> adjunct:AdverbPhrase VerbPhrase`,
  `crates/english/grammar/english.fug`), and spelling is substitution, so "also" is a Template
  over a subjectless phrase, with the subject supplied outside it. It adds no meaning: its body
  is its argument.
- Its hosts: the subjectless continuous-effect phrase of `syl-continuous-effect-subject`
  (*Efflorescence*, *Jetmir*, *Goatnap*); the Action a Handoff, an imperative or an event takes
  for a deed (*Liliana's Triumph*'s "each opponent also discards a card", *Goblin Barrage*, *Okk*'s
  "unless a creature with greater power also attacks"); and the sentence-initial "Also" before an
  imperative (*Beast Mode*'s "Also put a +1/+1 counter on that creature if …"), its own Template
  for its own placement. One Template per sort it wraps, added as a card needs it.
- Not a Template per verb (`alsoGain`, `alsoGet`, …), which multiplies with the 13 verbs the
  corpus puts "also" before; not a modifier argument of each verb, which would be a flag stored
  beside the term; and not the English layer's, since a Reading holds the word "also" and the
  English layer derives only inflection and agreement, case, voice and Gaps.
- The "is also a" type line and "is also goaded" are their own Templates beside the "in addition
  to its other types" forms.

## Done when

- The Templates above exist, each with a canon caller.
- *Efflorescence* writes "also" in its canon file, and every canon card that prints "also" in
  rules text writes it; the scan's canon section lists each.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

Writing the 57 non-canon cards. The `become` forms of "in addition to its other types":
`syl-become-replaces-types`.

## Pointers

`docs/decisions/spelling.md` (Constructions, substitution, the resolved term as the derivation);
`docs/decisions/english.md` §3, §7;
`plugins/canon/cards/Efflorescence.syl`; `plugins/builtin/templates/ability/isAlso.syl`;
`plugins/builtin/templates/action/become.syl`; `docs/evidence/syl-focus-also/`.
