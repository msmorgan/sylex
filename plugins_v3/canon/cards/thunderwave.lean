/-- Thunderwave -/
card thunderwave : Spelled :=
  { name := "Thunderwave", cost := [generic 2, pip .red, pip .red], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ rollDice 1 (.sides 20),
            applyResultsTable
              [ rollRow (oneThrough 9) (instruct this_ (dealDamage 3 (each isCreature))),
                rollRow (fromTo 10 19)
                  (sequentially
                    [ may (choose (a isCreature)),
                      instruct this_
                        (dealDamage 3 (each (and [isCreature, notChosen]))) ]),
                rollRow (exactly 20)
                  (instruct this_
                    (dealDamage 6 (each (and [isCreature, opponentsControl])))) ] ]) ] }
