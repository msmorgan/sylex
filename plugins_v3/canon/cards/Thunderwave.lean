/-- Thunderwave -/
card := {
  name := "Thunderwave",
  cost := [2, red, red],
  types := [sorcery],
  text := [
    spell
      [
        rollDice 1 (sides 20),
        applyResultsTable
          [
            rollRow (oneThrough 9) (instruct this_ (dealDamage 3 (each isCreature))),
            rollRow (fromTo 10 19)
              [
                may (choose (a isCreature)),
                instruct this_ (dealDamage 3 (each [isCreature, notChosen]))
              ],
            rollRow (exactly 20)
              (instruct this_ (dealDamage 6 (each [isCreature, opponentsControl])))
          ]
      ]
  ]
}
