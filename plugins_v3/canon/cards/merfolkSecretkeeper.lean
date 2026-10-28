card := adventurer
  {
    normal := {
      name := "Merfolk Secretkeeper",
      cost := [blue],
      types := [creature],
      subtypes := [merfolk, wizard],
      power := 0,
      toughness := 4
    },
    adventure := {
      name := "Venture Deeper",
      cost := [blue],
      types := [sorcery],
      subtypes := [adventure],
      text := [spell (instruct (target isPlayer) (mill 4))]
    }
  }
