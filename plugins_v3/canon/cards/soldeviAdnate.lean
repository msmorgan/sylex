/-- Soldevi Adnate -/
card := {
  name := "Soldevi Adnate",
  cost := [1, black],
  types := [creature],
  subtypes := [human, cleric],
  text := [
    activated (compound [tapSymbol, (sacrifice (a [isCreature, or [hasColor black, isArtifact]]))])
      (addMana (statOf (stat manaValue) (the [was (sacrifice which), isCreature]))
         (manaOf [[of black]]))
  ],
  power := 1,
  toughness := 2
}
