/-- Blind Fury -/
card blindFury : Spelled :=
  { name := "Blind Fury", cost := [generic 2, pip .red, pip .red], types := [.instant],
    text :=
      [ spell (sequentially
          [ establishThroughout (abilityLoss (allOf isCreature) [written trample]) untilEndOfTurn,
            establishThroughout
              (replace (would (a isCreature) (dealDamage anyAmount (a isCreature) (some .combat)))
                (instead (instruct it (dealDamage (times 2 thatMuch) (that creature)))))
              thisTurn ]) ] }
