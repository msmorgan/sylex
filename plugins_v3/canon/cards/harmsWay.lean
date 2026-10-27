card harmsWay : Spelled :=
  { name := "Harm's Way", cost := [pip .white], types := [.instant],
    text :=
      [ spell (establishThroughout
            (replace
              (would (aYourChoice isSource)
                (dealDamage anyAmount (andOr [you, allOf (and [isPermanent, youControl])])))
              (instead (redirectDamage (theNext 2) (target isAnyTarget))))
            thisTurn) ] }
