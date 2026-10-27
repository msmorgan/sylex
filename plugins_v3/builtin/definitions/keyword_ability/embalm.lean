keyword embalm (cost : Cost) : Ability := {
  lexical := {
    lemma := "embalm",
    parameters := [[Cost]],
    verbs := [{ lemma := "embalm", frames := [Monotransitive] }]
  },
  spelling := "embalm <cost>",
  body :=
    [Semantics.Templates.Ability.activatedOnlyDuring
        (Semantics.Templates.Cost.compound
          [cost,
            Semantics.Templates.Cost.perform
              (Semantics.Definitions.exileFrom Semantics.Templates.NounPhrase.thisCard Semantics.Templates.Location.yourGraveyard)])
        (Semantics.Definitions.create 1
          (Semantics.Templates.TokenSpec.copyOf Semantics.Templates.NounPhrase.this_
            [Semantics.Templates.CopyExcept.copyExceptEdits
                [Semantics.Templates.CharacteristicEdit.characteristicEditColors Semantics.QualityOp.sets
                    (Semantics.ColorSpec.some [Semantics.Color.white]),
                  Semantics.Templates.CharacteristicEdit.characteristicEditManaCost Option.none,
                  Semantics.Templates.CharacteristicEdit.characteristicEditTypeLine Semantics.QualityOp.adds
                    { subtypes := Option.some [Semantics.Definitions.zombie] }]]))
        Semantics.Templates.Timing.asSorcery]
}
