/-- Frenzy Sliver -/
card := {
  name := "Frenzy Sliver",
  cost := [1, black],
  types := [creature],
  subtypes := [sliver],
  text := [abilityGrant (allOf [sliver, isCreature]) (frenzy 1)],
  power := 1,
  toughness := 1
}
