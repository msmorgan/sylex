/-- Ghostfire Blade -/
card ghostfireBlade : Spelled :=
  { name := "Ghostfire Blade", cost := [generic 1], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (getsPt (equipped creature) (up 2) (up 2)),
        equip [generic 3],
        static (costShift
          (allOf (and [ abilityHead (.keyword equip), abilityOf this_,
                         targeting (a (and [isCreature, isColorless])) .someTarget ]))
          (less 2)) ] }
