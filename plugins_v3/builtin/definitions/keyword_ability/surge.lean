keyword surge (cost : Cost) : Ability := {
  lexical := { lemma := "surge", parameters := [[Cost]] },
  spelling := "surge <cost>",
  body :=
    [static
        (onlyWhile (altCost this_ (some cost))
          (did
            (actor
              (or
                [you, a isTeammate]))
            (cast (a anotherSpell))
            (within soFarThisTurn)))]
}
