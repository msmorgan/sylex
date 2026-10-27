/-- Approach of the Second Sun -/
card approachOfTheSecondSun : Spelled :=
  { name := "Approach of the Second Sun", cost := [generic 6, pip .white],
    types := [.sorcery],
    text :=
      [ spell (doIf
          (and
            [ is this_ (did any (cast which (from_ := yourHand)) thisObject),
              did you (cast (a
                (and [ isSpell, isOther,
                           named "Approach of the Second Sun" ]))) thisGame ])
          winGame
          (sequentially
            [ putInto this_ (fromTop 7 library), gainLife 7 ])) ] }
