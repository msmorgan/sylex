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
                [characteristicEditColors Semantics.QualityOp.sets
                    (Semantics.ColorSpec.some [Semantics.Color.white]),
                  characteristicEditManaCost none,
                  characteristicEditTypeLine Semantics.QualityOp.adds
                    { subtypes := some [zombie] }]]))
        asSorcery]
}
