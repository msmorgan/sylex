card := {
  name := "Rune Snag",
  cost := [1, blue],
  types := [instant],
  text := [
    spell
      (instruct (controllerOf (target isSpell))
         (doUnless (instruct you (counter it))
            (compound [[2], scaledGenericMana (forEach 2 [named "Rune Snag", in_ graveyard])])))
  ]
}
