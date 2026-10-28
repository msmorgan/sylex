card := {
  name := "Black Vise",
  cost := [1],
  types := [artifact],
  text := [
    entersChoosingPlayer thisArtifact anOpponent,
    at_ (beginningOfPossessed upkeep (the isChosenPlayer))
      [
        instruct thisArtifact (dealDamage (letter x) (that player)),
        define x (minus (countOf (cardIn (handOf they))) 4)
      ]
  ]
}
