/-- Termination Facilitator -/
card terminationFacilitator : Spelled :=
  { name := "Termination Facilitator", cost := [generic 1, pip .black], types := [.creature],
    subtypes := [human, assassin],
    text :=
      [ activatedOnlyDuring tapSymbol
          (putCounters 1 bountyCounter
            (target (or [isCreature, isPlaneswalker])))
          asSorcery,
        whenever
          (does any
             (dealDamage (to := a (and [or [isCreature, isPlaneswalker], anOpponentControls,
                      hasCounter bountyCounter]))))
          (destroy it) ],
    power := stat 1, toughness := stat 3 }
