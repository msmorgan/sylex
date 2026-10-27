card daughterOfAutumn : Spelled :=
  { name := "Daughter of Autumn", cost := [generic 2, pip .green, pip .green],
    supertypes := [.legendary], types := [.creature], subtypes := [avatar],
    text :=
      [ activated [pip .white]
          (establishThroughout
             (replace
               (would any (dealDamage anyAmount (target (and [isCreature, hasColor .white]))))
               (instead (redirectDamage (theNext 1) thisCreature)))
             thisTurn) ],
    power := stat 2, toughness := stat 4 }
