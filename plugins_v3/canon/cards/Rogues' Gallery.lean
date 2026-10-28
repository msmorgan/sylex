/-- Rogues' Gallery -/
card := {
  name := "Rogues' Gallery",
  cost := [2, black],
  types := [sorcery],
  text := [
    spell
      (doForEachKind color none color
         (returnToHand (targets (upTo 1) [isCreature, ofChosen color, in_ yourGraveyard])
            yourGraveyard))
  ]
}
