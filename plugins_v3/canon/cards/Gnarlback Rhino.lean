/-- Gnarlback Rhino -/
card := {
  name := "Gnarlback Rhino",
  cost := [2, green, green],
  types := [creature],
  subtypes := [rhino],
  text := [trample, whenever (does you (cast (a [isSpell, targeting thisCreature someTarget]))) (draw 1)],
  power := 4,
  toughness := 4
}
