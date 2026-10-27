keyword awaken (amount : Amount) (cost : Cost) : Ability := {
  lexical := { lemma := "awaken", parameters := [[Amount, DashCost]] },
  spelling := "awaken <amount>—<cost>",
  body :=
    [Semantics.Templates.Ability.static (Semantics.Templates.StaticSpec.altCost Semantics.Templates.NounPhrase.this_ (Option.some cost)),
      Semantics.Templates.Ability.spell
        (Semantics.Templates.Action.doIf
          (Semantics.Templates.Condition.keywordCostPaid Semantics.Definitions.awaken.label Semantics.Templates.NounPhrase.this_)
          (Semantics.Templates.Action.sequentially
            [Semantics.Templates.Action.putCounters amount (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
                (Semantics.Templates.NounPhrase.target
                  (Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isLand, Semantics.Templates.Predicate.youControl])),
              Semantics.Templates.Action.establish
                (Semantics.Templates.StaticSpec.characteristicChange (Semantics.Templates.NounPhrase.that Semantics.Templates.NounWord.land)
                  [Semantics.Templates.CharacteristicEdit.characteristicEditTypeLine Semantics.QualityOp.adds
                      { types := Option.some [Semantics.CardType.creature],
                        subtypes := Option.some [Semantics.Definitions.elemental] },
                    Semantics.Templates.CharacteristicEdit.characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.power 0,
                    Semantics.Templates.CharacteristicEdit.characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.toughness 0,
                    Semantics.Templates.CharacteristicEdit.characteristicEditAddedAbilities
                      [Semantics.Templates.keyword Semantics.Definitions.haste.label]])
                Option.none]))]
}
