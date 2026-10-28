card := {
  name := "Timely Reinforcements",
  cost := [2, white],
  types := [sorcery],
  text := [
    spell
      [
        doIf (compareAmt (lifeTotalOf you) less (lifeTotalOf anOpponent)) (gainLife 6),
        doIf
          (compareAmt (countOf [isCreature, youControl]) less
             (countOf [isCreature, anOpponentControls]))
          (create 3 (creatureToken 1 1 [white] [soldier]))
      ]
  ]
}
