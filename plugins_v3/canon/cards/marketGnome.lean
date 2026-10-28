/-- Market Gnome -/
card := {
  name := "Market Gnome",
  cost := [white],
  types := [artifact, creature],
  subtypes := [gnome],
  text := [
    when (does thisCreature die) [gainLife 1, draw 1],
    when (does any (exile thisCreature))
      (while_ := whileDoing (does you (activate (a (abilityHead (keyword craft))))))
      [gainLife 1, draw 1]
  ],
  power := 0,
  toughness := 3
}
