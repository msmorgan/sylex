/-- Blood Frenzy -/
card bloodFrenzy : Spelled :=
  { name := "Blood Frenzy", cost := [generic 1, pip .red], types := [.instant],
    text :=
      [ spell (sequentially
            [ get (target (and [isCreature, or [isAttacking, isBlocking]]))
                (up 4) (up 0) untilEndOfTurn,
              atNext .endStep (destroy (that creature)) ]) (timing :=
                                                                          beforePart
                                                                             .combatDamage none) ] }
