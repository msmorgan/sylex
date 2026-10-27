keyword readAhead : Ability := {
  lexical := { lemma := "read ahead" },
  spelling := "read ahead",
  body :=
    [ static
        (onlyWhile
          (cant
            (.actor
              (allOf
                (and
                  [ chapterAbilityOf thisSaga,
                    not (.compare [.chapter] .eq (countersOn loreCounter thisSaga)) ])))
            .trigger)
          (did thisSaga enter soFarThisTurn)) ]
}
