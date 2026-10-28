card := {
  name := "Thick-Skinned Goblin",
  cost := [1, red],
  types := [creature],
  subtypes := [goblin, shaman],
  text := [
    replace (would you (pay anyCost once paid (allOf [isPermanent, youControl]) (some echo)))
      (instead (may (pay [0] once))),
    activated [red] (gain thisCreature (protection (hasColor red)) untilEndOfTurn)
  ],
  power := 2,
  toughness := 1
}
