/-- Spirit of the Labyrinth -/
card spiritOfTheLabyrinth : Spelled :=
  { name := "Spirit of the Labyrinth", cost := [generic 1, pip .white],
    types := [.enchantment, .creature], subtypes := [spirit],
    text := [static (cant eachPlayer (drawCount (moreThan 1)) (timing := eachTurn))],
    power := stat 3, toughness := stat 1 }
