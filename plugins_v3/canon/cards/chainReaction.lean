card := {
  name := "Chain Reaction",
  cost := [2, red, red],
  types := [sorcery],
  text := [spell [instruct this_ (dealDamage (letter x) (each isCreature)), define x (countOf isCreature)]]
}
