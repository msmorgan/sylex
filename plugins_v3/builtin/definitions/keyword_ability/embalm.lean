keyword embalm (cost : Cost) : Ability := {
  lexical := {
    lemma := "embalm",
    parameters := [[Cost]],
    verbs := [{ lemma := "embalm", frames := [Monotransitive] }]
  },
  spelling := "embalm <cost>",
  body :=
    [activatedOnlyDuring
        (compound
          [cost,
            perform
              (exileFrom thisCard yourGraveyard)])
        (create 1
          (copyOf this_
            [copyExceptEdits
                [characteristicEditColors .sets
                    (.some [.white]),
                  characteristicEditManaCost none,
                  characteristicEditTypeLine .adds
                    { subtypes := some [zombie] }]]))
        asSorcery]
}
