card deflectingPalm : Spelled :=
  { name := "Deflecting Palm", cost := [pip .red, pip .white], types := [.instant],
    text :=
      [ spell
          (establishThroughout
            (replace (would (aYourChoice isSource) (dealDamage anyAmount you))
              (instead
                (preventDamage allDamage
                  (also := instruct this_ (dealDamage thatMuch (controllerOf thatSource)))))
              (use := .nextTimeOnly))
            thisTurn) ] }
