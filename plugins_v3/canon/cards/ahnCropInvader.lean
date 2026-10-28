/-- Ahn-Crop Invader -/
card := {
  name := "Ahn-Crop Invader",
  cost := [2, red],
  types := [creature],
  subtypes := [zombie, minotaur, warrior],
  text := [
    partScope turn you (abilityGrant thisCreature firstStrike),
    activated (compound [[1], (sacrifice (a [isCreature, isOther]))])
      (get thisCreature (up 2) (up 0) untilEndOfTurn)
  ],
  power := 2,
  toughness := 2
}
