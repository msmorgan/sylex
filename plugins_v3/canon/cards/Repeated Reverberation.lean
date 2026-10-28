card := {
  name := "Repeated Reverberation",
  cost := [2, red, red],
  types := [instant],
  text := [
    spell
      (delayWithin (does you (cast (a [isInstant, isSpell]))) thisTurn
         [copyOnStack (that stack) 2, may (chooseNewTargets (those copy))]
         [does you (cast (a [isSorcery, isSpell])), does you (activate (a (abilityHead loyalty)))])
  ]
}
