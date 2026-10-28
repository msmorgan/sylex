card := {
  name := "Inferno of the Star Mounts",
  cost := [4, red, red],
  supertypes := [legendary],
  types := [creature],
  subtypes := [dragon],
  text := [
    cant any (counter this_),
    flying,
    haste,
    activated [red]
      (triggerThisWay (get thisCreature (up 1) (up 0) untilEndOfTurn)
         (stateHolds (compareAmt (powerOf it) eq 20))
         (instruct it (dealDamage 20 (target isAnyTarget))))
  ],
  power := 6,
  toughness := 6
}
