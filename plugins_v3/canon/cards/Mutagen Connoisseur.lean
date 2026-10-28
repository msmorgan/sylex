card := {
  name := "Mutagen Connoisseur",
  cost := [1, green, blue],
  types := [creature],
  subtypes := [vedalken, mutant],
  text := [flying, vigilance, getsPt thisCreature (up (forEach 1 [isTransformed, youControl])) (up 0)],
  power := 0,
  toughness := 5
}
