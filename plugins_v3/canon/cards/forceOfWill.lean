card forceOfWill : Spelled :=
  { name := "Force of Will", cost := [generic 3, pip .blue, pip .blue], types := [.instant],
    text :=
      [ static (altCost this_
          (compound
            [ payLife 1,
              (exile (a (and [hasColor .blue, in_ yourHand]))) ])),
        spell (counter (target isSpell)) ] }
