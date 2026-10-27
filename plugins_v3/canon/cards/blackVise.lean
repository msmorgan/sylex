/-- Black Vise -/
card blackVise : Spelled :=
  { name := "Black Vise", cost := [generic 1], types := [.artifact],
    text :=
      [ static (entersChoosingPlayer thisArtifact anOpponent),
        at_ (beginningOfPossessed .upkeep (the isChosenPlayer))
          (sequentially
            [ instruct thisArtifact (dealDamage (letter .x) (that player)),
              define .x (minus (countOf (cardIn (handOf they))) 4) ]) ] }
