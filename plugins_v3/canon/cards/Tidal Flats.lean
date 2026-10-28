card := {
  name := "Tidal Flats",
  cost := [blue],
  types := [enchantment],
  text := [
    activated [blue, blue]
      (doForEach (each [isCreature, isAttacking, not (hasKeyword flying)])
         (instruct (controllerOf it)
            (doUnless
               (instruct you
                  (gain (allOf [isCreature, youControl, inCombat blockerOf (that creature)])
                     firstStrike untilEndOfTurn))
               [1])))
  ]
}
