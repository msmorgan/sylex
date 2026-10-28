keyword mayhem (cost : Cost) : Ability := {
  lexical := { lemma := "mayhem", parameters := [[Cost], []] },
  spelling := "mayhem <cost>",
  body :=
    [static
        (onlyWhile
          (can (actor you)
            (cast thisCard (some yourGraveyard)
              (payingInstead cost)))
          (did (actor you)
            (discard thisCard)
            (within soFarThisTurn)))]
}
