/-- "Read ahead" means "Chapter abilities of this Saga can't trigger the turn it entered the
battlefield unless it has exactly the number of lore counters on it specified in the chapter
symbol of that ability." [CR#702.155a] -/
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
                      (compare eq (countersOn loreCounter
                          thisSaga) [chapter])])))
            trigger)
          (did (actor thisSaga)
            enter (within soFarThisTurn)))]
}
