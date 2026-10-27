card divineDeflection : Spelled :=
  { name := "Divine Deflection", cost := [.variable, pip .white], types := [.instant],
    text :=
      [ spell (establishThroughout
            (replace
              (would any
                (dealDamage anyAmount (andOr [you, allOf (and [isPermanent, youControl])])))
              (instead
                (preventDamage (theNext (letter .x))
                  (also := instruct this_ (dealDamage thatMuch (target isAnyTarget))))))
            thisTurn) ] }
