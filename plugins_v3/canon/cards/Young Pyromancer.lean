card := {
  name := "Young Pyromancer",
  cost := [1, red],
  types := [creature],
  subtypes := [human, shaman],
  text := [
    whenever (does you (cast (a [isInstantOrSorcery, isSpell])))
      (create 1 (creatureToken 1 1 [red] [elemental]))
  ],
  power := 2,
  toughness := 1
}
