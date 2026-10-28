card := {
  name := "Bonus Round",
  cost := [1, red, red],
  types := [sorcery],
  text := [
    spell
      (delayWithin (does (a isPlayer) (cast (a [isInstantOrSorcery, isSpell]))) untilEndOfTurn
         [
           instruct (that player) (copyOnStack it),
           instruct (that player) (may (chooseNewTargets (that copy)))
         ])
  ]
}
