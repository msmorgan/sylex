card goblinSpy : Spelled :=
  { name := "Goblin Spy", cost := [pip .red], types := [.creature],
    subtypes := [goblin, rogue],
    text := [static (visibility .reveal you topOfLibrary)],
    power := stat 1, toughness := stat 1 }
