/-- "That source deals that damage to you instead": the same damage, from the same source, dealt
to another player, which is a redirection effect [CR#614.9]; `redirectDamage` keeps the source. -/
card jadeMonolith : Spelled :=
  { name := "Jade Monolith", cost := [generic 4], types := [.artifact],
    text :=
      [ activated [generic 1]
          (establishThroughout
            (replace (would (aYourChoice isSource) (dealDamage anyAmount (target isCreature)))
              (instead (redirectDamage allDamage you)) (use := .nextTimeOnly))
            thisTurn) ] }
