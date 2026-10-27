card healingGrace : Spelled :=
  { name := "Healing Grace", cost := [pip .white], types := [.instant],
    text :=
      [ spell (sequentially
          [ establishThroughout
              (replace
                (would (by_ (aYourChoice isSource)) (dealDamage anyAmount (target isAnyTarget)))
                (instead (preventDamage (theNext 3))))
              thisTurn,
            gainLife 3 ]) ] }
