/-- Leyline of Vitality -/
card leylineOfVitality : Spelled :=
  { name := "Leyline of Vitality", cost := [generic 2, pip .green, pip .green],
    types := [.enchantment],
    text :=
      [ mayBeginOnBattlefield,
        static (getsPt (allOf (and [isCreature, youControl])) (up 0) (up 1)),
        whenever (does (a (and [isCreature, youControl])) enter) (may (gainLife 1)) ] }
