card := {
  name := "Tuktuk the Explorer",
  cost := [2, red],
  supertypes := [legendary],
  types := [creature],
  subtypes := [goblin],
  text := [
    haste,
    when (does thisCreature die)
      (create 1
         (written
            {
              name := "Tuktuk the Returned",
              supertypes := [legendary],
              types := [artifact, creature],
              subtypes := [goblin, golem],
              power := 5,
              toughness := 5
            }))
  ],
  power := 1,
  toughness := 1
}
