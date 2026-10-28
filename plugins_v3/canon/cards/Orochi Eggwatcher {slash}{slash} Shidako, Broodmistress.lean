card := flip
  {
    normal := {
      name := "Orochi Eggwatcher",
      cost := [2, green],
      types := [creature],
      subtypes := [snake, shaman],
      text := [
        activated (compound [[2, green], tapSymbol])
          [
            create 1 (creatureToken 1 1 [green] [snake]),
            doIf (compareAmt (countOf [isCreature, youControl]) atLeast 10) (flip thisCreature)
          ]
      ],
      power := 1,
      toughness := 1
    },
    alternative := {
      name := "Shidako, Broodmistress",
      supertypes := [legendary],
      types := [creature],
      subtypes := [snake, shaman],
      text := [
        activated (compound [[green], sacrifice (a isCreature)])
          (get (target isCreature) (up 3) (up 3) untilEndOfTurn)
      ],
      power := 3,
      toughness := 3
    }
  }
