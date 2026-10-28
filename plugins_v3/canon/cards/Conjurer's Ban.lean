card := {
  name := "Conjurer's Ban",
  cost := [white, black],
  types := [sorcery],
  text := [
    spell
      [
        choose (a (quality cardName)),
        establishThroughout
          [
            cant any (cast (bare [isSpell, named chosenName])),
            cant any (play (allOf [isLand, named chosenName]))
          ]
          untilYourNextTurn
      ],
    spell (draw 1)
  ]
}
