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
                      (compare .eq (countersOn loreCounter
                          thisSaga) [.chapter])])))
            trigger)
          (did (actor thisSaga)
            enter (within soFarThisTurn)))]
}
