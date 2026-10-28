card := {
  name := "Chillerpillar",
  cost := [3, blue],
  supertypes := [snow],
  types := [creature],
  subtypes := [insect],
  text := [
    activated [4, snow, snow] (monstrosity 2),
    onlyWhile (abilityGrant thisCreature flying) (is thisCreature (hasDesignation monstrous))
  ],
  power := 3,
  toughness := 3
}
