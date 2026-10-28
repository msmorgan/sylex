/-- "Spectacle [cost]" means "You may pay [cost] rather than pay this spell's mana cost if an
opponent lost life this turn." [CR#702.137a] -/
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
