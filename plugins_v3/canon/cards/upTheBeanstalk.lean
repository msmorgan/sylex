/-- Up the Beanstalk -/
card upTheBeanstalk : Spelled :=
  { name := "Up the Beanstalk", cost := [generic 1, pip .green], types := [.enchantment],
    text :=
      [ when (does this_ enter)
          (joins := [joinedHead (does you (cast (a
            (and [isSpell, hasManaValue .atLeast 5]))))])
          (draw 1) ] }
