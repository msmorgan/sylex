/-- Katara, the Fearless -/
card kataraTheFearless : Spelled :=
  { name := "Katara, the Fearless", cost := [pip .green, pip .white, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, warrior, ally],
    text :=
      [ static (additionalTriggers
          (triggers (a (and [ abilityHead .anyTriggered,
                       abilityOf (a (and [ ally,
                                             youControl ])) ])))
          (exactly 1)) ],
    power := stat 3, toughness := stat 3 }
