/-- Kopala, Warden of Waves -/
card kopalaWardenOfWaves : Spelled :=
  { name := "Kopala, Warden of Waves", cost := [generic 1, pip .blue, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [merfolk, wizard],
    text :=
      [ static (costShift
          (allOf (and [ isSpell, did yourOpponents (cast which) thisObject,
                         targeting (a (and [isCreature, merfolk, youControl]))
                           .someTarget ]))
          (more 2)),
        static (costShift
          (allOf (and [ abilityHead .anyActivated, did yourOpponents (activate which) thisObject,
                         targeting (a (and [isCreature, merfolk, youControl]))
                           .someTarget ]))
          (more 2)) ],
    power := stat 2, toughness := stat 2 }
