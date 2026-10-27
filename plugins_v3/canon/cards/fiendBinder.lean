/-- Fiend Binder -/
card fiendBinder : Spelled :=
  { name := "Fiend Binder", cost := [generic 3, pip .white], types := [.creature],
    subtypes := [human, soldier],
    text :=
      [ whenever (does thisCreature attack)
          (tap
            (target (and [isCreature, controlledBy defendingPlayer]))) ],
    power := stat 3, toughness := stat 2 }
