card repeal : Spelled :=
  { name := "Repeal", cost := [.variable, pip .blue], types := [.instant],
    text :=
      [ spell (sequentially
          [ returnToHand (target (and [non isLand, isPermanent, hasManaValue .eq (letter .x)])),
            draw 1 ]) ] }
