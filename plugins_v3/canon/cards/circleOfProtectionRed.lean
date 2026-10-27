card circleOfProtectionRed : Spelled :=
  { name := "Circle of Protection: Red", cost := [generic 1, pip .white], types := [.enchantment],
    text :=
      [ activated [generic 1]
          (establishThroughout
             (replace
               (would (aYourChoice (and [isSource, hasColor .red])) (dealDamage anyAmount you))
               (instead (preventDamage allDamage)) (use := .nextTimeOnly))
             thisTurn) ] }
