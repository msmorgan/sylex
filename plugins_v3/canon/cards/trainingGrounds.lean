card trainingGrounds : Spelled :=
  { name := "Training Grounds", cost := [pip .blue], types := [.enchantment],
    text :=
      [ static (costShift (allOf (and [abilityHead .anyActivated, abilityOf (allOf
          (and [isCreature, youControl]))]))
          (less 2 (some 1))) ] }
