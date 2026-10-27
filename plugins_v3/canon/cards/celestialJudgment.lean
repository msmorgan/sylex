/-- Celestial Judgment -/
card celestialJudgment : Spelled :=
  { name := "Celestial Judgment", cost := [generic 4, pip .white, pip .white],
    types := [.sorcery],
    text :=
      [ spell (sequentially
          [ doForEachKind (.value .power) .number (choose (a (and [isCreature, hasPower .eq chosenNumber]))) (allOf isCreature),
            destroy (each (and [isCreature, notChosen])) ]) ] }
