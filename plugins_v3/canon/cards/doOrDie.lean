/-- Do or Die -/
card doOrDie : Spelled :=
  { name := "Do or Die", cost := [generic 1, pip .black], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ separateIntoPiles
              (allOf (and [isCreature, controlledBy (target isPlayer)])) 2,
            destroyCantRegenerate
              (allOf (and [isCreature, inPile (pileOfChoice they)])) ]) ] }
