---
needs: []
---
# `discourse`: a card a cast permission names is "that spell" to the text after it

Standard constraints apply. Split from `lean-v3-cast-spell-anaphora-and-noted-mana` (its former
Part 1, design item 2), which keeps the AST change to `SpendPurpose.toCast`. Found by the
2026-10-25 triage of an external audit ("Atraxa"), verified against `wmyykqnv`.

## The cards

- *Rogue Class* level 3: "You may play cards exiled with this Class, and you may spend mana as
  though it were mana of any color to cast those spells."
- *Summon: Esper Valigarmanda* chapters II-IV: "You may cast an instant or sorcery card exiled
  with this Saga, and mana of any type can be spent to cast that spell."

Both are fragments today (`rogueClassLevelThree`, `summonEsperValigarmandaCast` in
`lean/Semantics/Proofs/CardsChoice.lean`, carried to Rust tests by `semantics-rust-rewrite`).
Both re-describe the anaphor (`toCast (in_ (exiledWith thisClass))`, `toCast (and
[isInstantOrSorcery, in_ (exiledWith thisSaga)])`) because the Lean checker refuses it even in a
NounPhrase slot. Probed: `conjunction [can you (play (allOf (cardIn (exiledWith thisClass)))), can
you (cast (those .spell))]` gives `[.anaphor (.word .spell) .many 0]`; Esper's `cast (that
.spell)` gives `[.anaphor (.word .spell) .one 0]`. (`cast them` checks `[]`, but drops the printed
"spells"; do not use it.)

## Requirement

Casting moves the card to the stack, where it is the spell [CR#601.2a]. `discourse` already treats
a performed `play`/`cast` this way (the referent is stamped with the procedure deed, so the
`.spell` head reaches it; Lean's pin `playRecordsALandPlayOrASpellCast`). It must do the same for
a `can` permission over `cast`/`play`: when a `conjunction` (or a `doIf` continuation) steps
`can <who> (cast|play X)`, X's referent is published to the later conjuncts with the procedure
deed stamped, so the `.spell` head (and `.subtype (.spell _)`) resolves to it as after a performed
cast. The permission's own `allOf …` or `a …` determiner fixes plurality ("those spells" for
*Rogue Class*'s `allOf`, "that spell" for Esper's `a`). Nothing else about a permission's
referents changes; a `can` outside a conjunction publishes nothing new.

## Respell the two cards

- *Rogue Class*: `conjunction [ can you (play (allOf (cardIn (exiledWith thisClass)))), can you
  (pay anyCost .once) (asThough := some (spendManaAs none .anyColor (toCast (those .spell)))) ]`.
  The `cardIn` head noun is also `lean-v3-card-head-noun-sweep` §1 row 5's; whichever lands
  second keeps it.
- *Esper*: `conjunction [ can you (cast (a (and [isInstantOrSorcery, isCard, in_ (exiledWith
  thisSaga)]))), can you (pay anyCost .once) (asThough := some (spendManaAs none .anyType (toCast
  (that .spell)))) ]`.

`toCast` over a NounPhrase is `lean-v3-cast-spell-anaphora-and-noted-mana`'s. If it has not
landed, test the requirement through a NounPhrase slot (`cast (that .spell)`) and leave the two
`toCast` respellings to whichever of the two tickets lands second.

## Tests (`discourse`)

- *Rogue Class* and *Esper* as above: each checks, and the second conjunct resolves to the first's
  referent (was the proposed Lean pin `castPermissionPublishesThatSpell`).
- `can you (pay anyCost .once) (asThough := some (spendManaAs none .anyColor (toCast (those
  .spell))))` alone is refused with an unresolved `.spell` anaphor, plural (was
  `thoseSpellsNeedThePermission`).
- Must stay green: the carried-over tests for a performed play/cast
  (`playRecordsALandPlayOrASpellCast`) and for keyword cast permissions
  (`Proofs/KeywordCastPermissions.lean`).
