card := {
  name := "Override",
  cost := [2, blue],
  types := [instant],
  text := [
    spell
      (instruct (controllerOf (target isSpell))
         (doUnless (instruct you (counter it))
            (scaledGenericMana (forEach 1 [isArtifact, youControl]))))
  ]
}
