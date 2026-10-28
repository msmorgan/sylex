card := {
  name := "Goblin Electromancer",
  cost := [blue, red],
  types := [creature],
  subtypes := [goblin, wizard],
  text := [costShift (allOf [isInstantOrSorcery, isSpell, did you (cast which) thisObject]) (less 1)],
  power := 2,
  toughness := 2
}
