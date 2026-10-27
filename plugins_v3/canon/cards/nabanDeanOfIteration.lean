/-- Naban, Dean of Iteration -/
card nabanDeanOfIteration : Spelled :=
  { name := "Naban, Dean of Iteration", cost := [generic 1, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, wizard],
    text :=
      [ static (additionalTriggers
          (causes
            (byEvent
              (does (a (and [ wizard,
                                  youControl ])) enter))
            (triggers (a (and [ abilityHead .anyTriggered,
                         abilityOf (a (and [isPermanent, youControl])) ]))))
          (exactly 1)) ],
    power := stat 2, toughness := stat 1 }
