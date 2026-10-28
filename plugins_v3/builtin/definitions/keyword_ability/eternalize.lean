keyword eternalize (cost : Cost) : Ability := {
  lexical := { lemma := "eternalize", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): eternalize [mana cost] / eternalize—[verbal cost]; separator and terminal
  -- period depend on the Cost structure.
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
                    (Semantics.ColorSpec.some [Semantics.Color.black]),
                  characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.power 4,
                  characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.toughness 4,
                  characteristicEditManaCost none,
                  characteristicEditTypeLine Semantics.QualityOp.adds
                    { subtypes := some [zombie] }]]))
        asSorcery]
}
