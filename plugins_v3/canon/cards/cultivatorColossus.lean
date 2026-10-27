card cultivatorColossus : Spelled :=
  { name := "Cultivator Colossus", cost := [generic 4, pip .green, pip .green, pip .green],
    types := [.creature], subtypes := [plant, beast],
    text :=
      [ trample,
        static (ptDefinition thisCreature .bothEach
          (countOf (and [isLand, youControl]))),
        when (does thisCreature enter)
          (may (putOntoBattlefieldTapped (a (and [isLand, in_ yourHand])))
            (sequentially [draw 1, repeat_ again])) ] }
