keyword eternalize (cost : Cost) : Ability := {
  lexical := { lemma := "eternalize", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): eternalize [mana cost] / eternalize—[verbal cost]; separator and terminal
  -- period depend on the Cost structure.
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
                    (Semantics.ColorSpec.some [Semantics.Color.black]),
                  Semantics.Templates.CharacteristicEdit.characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.power 4,
                  Semantics.Templates.CharacteristicEdit.characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.toughness 4,
                  Semantics.Templates.CharacteristicEdit.characteristicEditManaCost Option.none,
                  Semantics.Templates.CharacteristicEdit.characteristicEditTypeLine Semantics.QualityOp.adds
                    { subtypes := Option.some [Semantics.Definitions.zombie] }]]))
        Semantics.Templates.Timing.asSorcery]
}
