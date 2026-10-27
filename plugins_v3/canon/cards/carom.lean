card carom : Spelled :=
  { name := "Carom", cost := [generic 1, pip .white], types := [.instant],
    text :=
      [ spell (sequentially
          [ establishThroughout
              (replace (would any (dealDamage anyAmount (target isCreature)))
                (instead (redirectDamage (theNext 1) (target (and [isCreature, isOther])))))
              thisTurn,
            draw 1 ]) ] }
