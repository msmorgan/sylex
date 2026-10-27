/-- Menacing Ogre -/
card menacingOgre : Spelled :=
  { name := "Menacing Ogre", cost := [generic 3, pip .red, pip .red], types := [.creature],
    subtypes := [ogre],
    text :=
      [ trample, haste,
        when (does thisCreature enter)
          (sequentially
            [ instruct eachPlayer (choose (disclosure := .secretly) (a (quality .number))),
              revealChoices .numbers,
              instruct (each (and [isPlayer, choseExtreme .max])) (loseLife thatMuch),
              doIf (is you (choseExtreme .max))
                (putCounters 2 p1p1Counter thisCreature) ]) ],
    power := stat 3, toughness := stat 3 }
