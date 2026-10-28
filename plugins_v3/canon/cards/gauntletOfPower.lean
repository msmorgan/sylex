/-- Gauntlet of Power -/
card := {
  name := "Gauntlet of Power",
  cost := [5],
  types := [artifact],
  text := [
    entersChoosing thisArtifact color,
    getsPt (allOf [isCreature, ofChosen color]) (up 1) (up 1),
    whenever (does any (tapForMana (a [isLand, hasSupertype basic]) (some (ofColor thatColor))))
      (instruct (controllerOf (that land)) (addMana 1 ofTheChosenColor))
  ]
}
