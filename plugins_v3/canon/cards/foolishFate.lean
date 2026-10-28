/-- Foolish Fate: "Destroy target creature. / Infusion — If you gained life this turn, that
creature's controller loses 3 life." "That creature" reads the target as the first ability left
it. -/
card := {
  name := "Foolish Fate",
  cost := [2, black],
  types := [instant],
  text := [
    spell (destroy (target isCreature)),
    infusion
      (spell
         (doIf (did you (gainLife anyAmount) soFarThisTurn)
            (instruct (controllerOf (that creature)) (loseLife 3))))
  ]
}
