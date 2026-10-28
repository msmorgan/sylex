/-- Sanctum Prelate -/
card := {
  name := "Sanctum Prelate",
  cost := [1, white, white],
  types := [creature],
  subtypes := [human, cleric],
  text := [
    entersChoosing thisCreature number,
    cant any (cast (bare [isSpell, non isCreature, hasManaValue eq chosenNumber]))
  ],
  power := 2,
  toughness := 2
}
