/-- Foriysian Brigade -/
card := {
  name := "Foriysian Brigade",
  cost := [3, white],
  types := [creature],
  subtypes := [human, soldier],
  text := [can thisCreature (block (counted (additional (exactly 1)) isCreature)) (timing := eachCombat)],
  power := 2,
  toughness := 4
}
