/-- Blood Reckoning -/
card bloodReckoning : Spelled :=
  { name := "Blood Reckoning", cost := [generic 3, pip .black], types := [.enchantment],
    text :=
      [ whenever
          (does (a isCreature)
             (attack (youOr (a (and [isPlaneswalker, youControl])))))
          (instruct (controllerOf (that creature)) (loseLife 1)) ] }
