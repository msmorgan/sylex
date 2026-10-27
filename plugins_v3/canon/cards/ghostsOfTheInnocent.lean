card ghostsOfTheInnocent : Spelled :=
  { name := "Ghosts of the Innocent", cost := [generic 5, pip .white, pip .white],
    types := [.creature], subtypes := [spirit],
    text :=
      [ static (replace (would (a isSource) (dealDamage anyAmount (a (or [isPermanent, isPlayer]))))
          (instead (instruct it (dealDamage (half .down thatMuch) thatJoin)))) ],
    power := stat 4, toughness := stat 5 }
