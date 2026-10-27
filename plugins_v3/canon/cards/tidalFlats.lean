card tidalFlats : Spelled :=
  { name := "Tidal Flats", cost := [pip .blue], types := [.enchantment],
    text :=
      [ activated [pip .blue, pip .blue]
          (doForEach (each (and [isCreature, isAttacking, not (hasKeyword flying)]))
            (instruct (controllerOf it) (doUnless (instruct you (gain
                (allOf (and [ isCreature, youControl,
                               inCombat .blockerOf (that creature) ]))
                firstStrike untilEndOfTurn)) [generic 1]))) ] }
