/-- Blaring Captain -/
card blaringCaptain : Spelled :=
  { name := "Blaring Captain", cost := [generic 3, pip .black], types := [.creature],
    subtypes := [azra, warrior],
    text :=
      [ partnerWith (named "Blaring Recruiter"),
        whenever (does thisCreature attack)
          (get (allOf (and [warrior, isAttacking])) (up 1) (up 1) untilEndOfTurn) ],
    power := stat 2, toughness := stat 2 }
