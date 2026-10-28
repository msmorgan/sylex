card := {
  name := "Fork",
  cost := [red, red],
  types := [instant],
  text := [
    spell
      [
        copyOnStack (target [isInstantOrSorcery, isSpell]) (exceptions := [isColor red]),
        may (chooseNewTargets (that copy))
      ]
  ]
}
