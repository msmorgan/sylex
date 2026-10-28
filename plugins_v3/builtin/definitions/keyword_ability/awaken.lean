keyword awaken (amount : Amount) (cost : Cost) : Ability := {
  lexical := { lemma := "awaken", parameters := [[Amount, DashCost]] },
  spelling := "awaken <amount>—<cost>",
  body :=
    [static (altCost this_ (some cost)),
      spell
        (doIf
          (keywordCostPaid awaken.label this_)
          (sequentially
            [putCounters amount (printedKind p1p1Counter)
                (target
                  (and [isLand, youControl])),
              establish
                (characteristicChange (that land)
                  [characteristicEditTypeLine Semantics.QualityOp.adds
                      { types := some [Semantics.CardType.creature],
                        subtypes := some [elemental] },
                    characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.power 0,
                    characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.toughness 0,
                    characteristicEditAddedAbilities
                      [keyword haste.label]])
                none]))]
}
