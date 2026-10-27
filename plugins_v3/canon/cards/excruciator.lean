card excruciator : Spelled :=
  { name := "Excruciator", cost := [generic 6, pip .red, pip .red], types := [.creature],
    subtypes := [avatar],
    text :=
      [ static
          (cant any
            (preventDamage allDamage
              (damage := would (by_ thisCreature) (dealDamage anyAmount)))) ],
    power := stat 7, toughness := stat 7 }
