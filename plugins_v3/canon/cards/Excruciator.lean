card := {
  name := "Excruciator",
  cost := [6, red, red],
  types := [creature],
  subtypes := [avatar],
  text := [cant any (preventDamage allDamage (damage := would (by_ thisCreature) (dealDamage anyAmount)))],
  power := 7,
  toughness := 7
}
