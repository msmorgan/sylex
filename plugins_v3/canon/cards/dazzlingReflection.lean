card dazzlingReflection : Spelled :=
  { name := "Dazzling Reflection", cost := [generic 1, pip .white], types := [.instant],
    text :=
      [ spell (sequentially
          [ gainLife (powerOf (target isCreature)),
            establishThroughout
              (replace (would (that creature) (dealDamage anyAmount))
                (instead (preventDamage allDamage)) (use := .nextTimeOnly))
              thisTurn ]) ] }
