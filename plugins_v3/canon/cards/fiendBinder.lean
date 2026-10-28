/-- Fiend Binder -/
card := {
  name := "Fiend Binder",
  cost := [3, white],
  types := [creature],
  subtypes := [human, soldier],
  text := [whenever (does thisCreature attack) (tap (target [isCreature, controlledBy defendingPlayer]))],
  power := 3,
  toughness := 2
}
