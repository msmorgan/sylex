card := {
  name := "Harried Spearguard",
  cost := [red],
  types := [creature],
  subtypes := [human, soldier],
  text := [
    haste,
    when (does thisCreature die)
      (create 1
         (written
            {
              colors := [black],
              types := [creature],
              subtypes := [rat],
              text := [cant thisToken block],
              power := 1,
              toughness := 1
            }))
  ],
  power := 1,
  toughness := 1
}
