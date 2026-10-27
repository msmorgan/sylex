card fireServant : Spelled :=
  { name := "Fire Servant", cost := [generic 3, pip .red, pip .red], types := [.creature],
    subtypes := [elemental],
    text :=
      [ static
          (replace
            (would (a (and [isInstantOrSorcery, hasColor .red, youControl])) (dealDamage anyAmount))
            (instead (instruct it (dealDamage (times 2 thatMuch))))) ],
    power := stat 4, toughness := stat 3 }
