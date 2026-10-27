card shadowbane : Spelled :=
  { name := "Shadowbane", cost := [generic 1, pip .white], types := [.instant],
    text :=
      [ spell (establishThroughout
            (replace
              (would (aYourChoice isSource)
                (dealDamage anyAmount (andOr [you, allOf (and [isCreature, youControl])])))
              (instead
                (preventDamage allDamage
                  (also := doIf (did any (preventDamage allDamage
                                    (damage := would (a (and [isSource, hasColor .black]))
                                      (dealDamage anyAmount)))
                                  thisWay)
                    (gainLife thatMuch))))
              (use := .nextTimeOnly))
            thisTurn) ] }
