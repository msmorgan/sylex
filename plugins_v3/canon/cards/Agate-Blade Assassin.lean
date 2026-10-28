card := {
  name := "Agate-Blade Assassin",
  cost := [1, black],
  types := [creature],
  subtypes := [lizard, assassin],
  text := [whenever (does thisCreature attack) [instruct defendingPlayer (loseLife 1), gainLife 1]],
  power := 1,
  toughness := 3
}
