card mirrorwoodTreefolk : Spelled :=
  { name := "Mirrorwood Treefolk", cost := [generic 3, pip .green], types := [.creature],
    subtypes := [treefolk],
    text :=
      [ activated [generic 2, pip .red, pip .white]
          (establishThroughout
            (replace (would any (dealDamage anyAmount thisCreature))
              (instead (redirectDamage allDamage (target isAnyTarget))) (use := .nextTimeOnly))
            thisTurn) ],
    power := stat 2, toughness := stat 4 }
