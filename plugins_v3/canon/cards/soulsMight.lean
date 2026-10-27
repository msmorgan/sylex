/-- Soul's Might -/
card soulsMight : Spelled :=
  { name := "Soul's Might", cost := [generic 4, pip .green], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ putCounters (letter .x) p1p1Counter (target isCreature),
            define .x (powerOf (that creature)) ]) ] }
