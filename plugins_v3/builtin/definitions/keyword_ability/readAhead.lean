keyword readAhead : Ability := {
  lexical := { lemma := "read ahead" },
  spelling := "read ahead",
  body :=
    [static
        (onlyWhile
          (cant
            (actor
              (allOf
                (and
                  [chapterAbilityOf thisSaga,
                    not
                      (predicateCompare [.chapter] .eq
                        (countersOn loreCounter
                          thisSaga))])))
            trigger)
          (did (actor thisSaga)
            enter (within soFarThisTurn)))]
}
