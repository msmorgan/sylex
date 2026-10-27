/-- Embercleave -/
card embercleave : Spelled :=
  { name := "Embercleave", cost := [generic 4, pip .red, pip .red], supertypes := [.legendary],
    types := [.artifact], subtypes := [equipment],
    text :=
      [ flash,
        static (costShift this_
          (less (forEach 1 (and [isCreature, isAttacking, youControl])))),
        when (does thisEquipment enter) (attachTo it (target (and [isCreature, youControl]))),
        static (conjunction [ getsPt (equipped creature) (up 1) (up
                1),
            abilityGrant (equipped creature) doubleStrike,
            abilityGrant (equipped creature) trample ] (subject :=
                                                                      equipped
                                                                         creature)),
        equip [generic 3] ] }
