keyword mayhem (cost : Cost) : Ability := {
  lexical := { lemma := "mayhem", parameters := [[Cost], []] },
  spelling := "mayhem <cost>",
  body :=
    [ static
        (onlyWhile
          (can you
            (cast thisCard (from_ := yourGraveyard)
              (payment := .payingInstead cost)))
          (did you (discard thisCard) soFarThisTurn)) ]
}
