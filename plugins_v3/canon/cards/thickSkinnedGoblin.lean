card thickSkinnedGoblin : Spelled :=
  { name := "Thick-Skinned Goblin", cost := [generic 1, pip .red], types := [.creature],
    subtypes := [goblin, shaman],
    text :=
      [ thickSkinnedGoblinEchoPayment,
        activated [pip .red]
          (gain thisCreature (protection (hasColor .red)) untilEndOfTurn) ],
    power := stat 2, toughness := stat 1 }
