card hellkiteCharger : Spelled :=
  { name := "Hellkite Charger", cost := [generic 4, pip .red, pip .red],
    types := [.creature], subtypes := [dragon],
    text :=
      [ flying, haste,
        whenever (does thisCreature attack)
          (may (pay [generic 5, pip .red, pip .red] .once)
            (sequentially
              [ untap (allOf (and [isCreature, isAttacking])),
                additionalPhase .combat none 1 ])) ],
    power := stat 5, toughness := stat 5 }
