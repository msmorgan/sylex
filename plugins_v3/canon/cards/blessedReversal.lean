/-- Blessed Reversal -/
card := {
  name := "Blessed Reversal",
  cost := [1, white],
  types := [instant],
  text := [spell (gainLife (forEach 3 [isCreature, inCombat attackerOf you]))]
}
