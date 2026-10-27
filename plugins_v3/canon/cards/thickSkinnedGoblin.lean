card thickSkinnedGoblin : Spelled :=
  { name := "Thick-Skinned Goblin", cost := [generic 1, pip .red], types := [.creature],
    subtypes := [goblin, shaman],
    text :=
      [ (static (replace
            (would you (pay anyCost .once .paid (allOf (and [isPermanent, youControl]))
              (some echo)))
            (instead (may (pay [generic 0] .once))))),
        activated [pip .red]
          (gain thisCreature (protection (hasColor .red)) untilEndOfTurn) ],
    power := stat 2, toughness := stat 1 }
