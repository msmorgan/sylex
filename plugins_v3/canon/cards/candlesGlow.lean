card candlesGlow : Spelled :=
  { name := "Candles' Glow", cost := [generic 1, pip .white], types := [.instant],
    subtypes := [arcane],
    text :=
      [ spell
          (establishThroughout
            (replace (would any (dealDamage anyAmount (target isAnyTarget)))
              (instead (preventDamage (theNext 3) (also := gainLife preventedThisWay))))
            thisTurn),
        Ability.splice (hasSubtype arcane) [generic 1, pip .white] ] }
