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
                      (predicateCompare [Semantics.ProjAxis.chapter] Semantics.Comparator.eq
                        (countersOn loreCounter
                          thisSaga))])))
            trigger)
          (did (actor thisSaga)
            enter (within soFarThisTurn)))]
}
