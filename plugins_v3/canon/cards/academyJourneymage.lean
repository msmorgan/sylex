/-- Academy Journeymage -/
card academyJourneymage : Spelled :=
  { name := "Academy Journeymage", cost := [generic 4, pip .blue], types := [.creature],
    subtypes := [human, wizard],
    text :=
      [ static (onlyIfSo (costShift this_ (less 1))
          (exists_ (and [wizard, youControl]))),
        when (does thisCreature enter)
          (returnToHand (target (and [isCreature, anOpponentControls]))) ],
    power := stat 3, toughness := stat 2 }
