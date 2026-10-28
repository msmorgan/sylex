card := {
  name := "Belltower Sphinx",
  cost := [4, blue],
  types := [creature],
  subtypes := [sphinx],
  text := [
    flying,
    whenever (does (a isSource) (dealDamage anyAmount thisCreature))
      (instruct (controllerOf thatSource) (mill thatMuch))
  ],
  power := 2,
  toughness := 5
}
