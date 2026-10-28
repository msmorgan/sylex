keyword spectacle (cost : Cost) : Ability := {
  lexical := { lemma := "spectacle", parameters := [[Cost]] },
  spelling := "spectacle <cost>",
  body :=
    [static
        (onlyWhile (altCost this_ (some cost))
          (did (actor anOpponent)
            (loseLife anyAmount)
            (within soFarThisTurn)))]
}
