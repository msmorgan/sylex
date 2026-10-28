card := {
  name := "Bloodfire Enforcers",
  cost := [3, red],
  types := [creature],
  subtypes := [human, monk],
  text := [
    onlyWhile [abilityGrant thisCreature firstStrike, abilityGrant it trample]
      [exists_ [isInstant, in_ yourGraveyard], exists_ [isSorcery, in_ yourGraveyard]]
  ],
  power := 5,
  toughness := 2
}
