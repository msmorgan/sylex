card templeAltisaur : Spelled :=
  { name := "Temple Altisaur", cost := [generic 4, pip .white], types := [.creature],
    subtypes := [dinosaur],
    text :=
      [ static (replace (would (a isSource) (dealDamage anyAmount (a (and [dinosaur, youControl, isOther]))))
          (instead (preventDamage (allBut 1)))) ],
    power := stat 3, toughness := stat 4 }
