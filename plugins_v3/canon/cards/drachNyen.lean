card drachNyen : Spelled :=
  { name := "Drach'Nyen", cost := [generic 4, pip .black, pip .red],
    supertypes := [.legendary], types := [.artifact], subtypes := [equipment],
    text :=
      [ when (does thisEquipment enter) (exile (targets (upTo 1) isCreature)),
        static (conjunction
                  [ abilityGrant (equipped creature) menace,
            getsPt (equipped creature) (up (letter .x)) (up 0),
            letterDefinition .x (powerOf (the (in_ (exiledWith thisEquipment)))) ]),
        equip [generic 2] ] }
