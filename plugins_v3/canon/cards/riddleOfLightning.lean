card riddleOfLightning : Spelled :=
  { name := "Riddle of Lightning", cost := [generic 3, pip .red, pip .red], types := [.instant],
    text :=
      [ spell (sequentially
          [ choose (target isAnyTarget),
            scry 3,
            reveal (topSlice 1),
            instruct this_
                (dealDamage (statOf (.stat .manaValue) (that card)) thatJoin) ]) ] }
