/-- Gauntlet of Power -/
card gauntletOfPower : Spelled :=
  { name := "Gauntlet of Power", cost := [generic 5], types := [.artifact],
    text :=
      [ static (entersChoosing thisArtifact .color),
        static (getsPt (allOf (and [isCreature, ofChosen .color])) (up 1) (up 1)),
        whenever
          (does any (tapForMana (a (and [isLand, hasSupertype .basic])) (some (ofColor thatColor))))
          (instruct (controllerOf (that land)) (addMana 1 ofTheChosenColor)) ] }
