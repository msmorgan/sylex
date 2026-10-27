/-- Explosive Singularity -/
card explosiveSingularity : Spelled :=
  { name := "Explosive Singularity", cost := [generic 8, pip .red, pip .red], types := [.sorcery],
    text :=
      [ static (addedCost
          (tap (counted anyNumber (and [isCreature, youControl, isUntapped])))
          true),
        static (costShift this_ (less (times 1 groupSize))),
        spell (instruct this_ (dealDamage 10 (target isAnyTarget))) ] }
