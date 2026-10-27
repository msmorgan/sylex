card incinerate : Spelled :=
  { name := "Incinerate", cost := [generic 1, pip .red], types := [.instant],
    text :=
      [ spell (sequentially
          [ instruct this_ (dealDamage 3 (target isAnyTarget)),
            establishThroughout (cant any
                                   (regeneration
                                      (a (and
                                            [ isCreature,
                                              was (dealDamage anyAmount which) thisWay ]))))
              thisTurn ]) ] }
