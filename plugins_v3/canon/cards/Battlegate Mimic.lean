/-- Battlegate Mimic -/
card := {
  name := "Battlegate Mimic",
  cost := [1, hybridPip red white],
  types := [creature],
  subtypes := [shapeshifter],
  text := [
    whenever (does you (cast (a [isSpell, hasColor red, hasColor white])))
      (establishThroughout [getsBase thisCreature 4 2, abilityGrant thisCreature firstStrike]
         untilEndOfTurn)
  ],
  power := 2,
  toughness := 1
}
