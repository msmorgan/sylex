card := {
  name := "Deflecting Palm",
  cost := [red, white],
  types := [instant],
  text := [
    spell
      (establishThroughout
         (replace (would (aYourChoice isSource) (dealDamage anyAmount you))
            (instead
               (preventDamage allDamage
                  (also := instruct this_ (dealDamage thatMuch (controllerOf thatSource)))))
            (use := nextTimeOnly))
         thisTurn)
  ]
}
