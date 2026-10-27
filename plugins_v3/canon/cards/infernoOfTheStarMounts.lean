card infernoOfTheStarMounts : Spelled :=
  { name := "Inferno of the Star Mounts", cost := [generic 4, pip .red, pip .red],
    supertypes := [.legendary], types := [.creature], subtypes := [dragon],
    text :=
      [ static (cant any (counter this_)),
        flying,
        haste,
        activated [pip .red]
          (triggerThisWay (get thisCreature (up 1) (up 0) untilEndOfTurn)
            (stateHolds (compareAmt (powerOf it) .eq 20))
            (instruct it (dealDamage 20 (target isAnyTarget)))) ],
    power := stat 6, toughness := stat 6 }
