/-- Scrapbasket -/
card scrapbasket : Spelled :=
  { name := "Scrapbasket", cost := [generic 4], types := [.artifact, .creature],
    subtypes := [scarecrow],
    text := [activated [generic 1] (becomeColor thisCreature .every
        untilEndOfTurn)],
    power := stat 3, toughness := stat 2 }
