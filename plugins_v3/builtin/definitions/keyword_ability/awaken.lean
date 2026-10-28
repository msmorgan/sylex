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
                  [typeLine .adds
                      { types := some [Semantics.CardType.creature],
                        subtypes := some [elemental] },
                    stat .sets .power 0,
                    stat .sets .toughness 0,
                    addedAbilities
                      [keyword haste.label]])
                none]))]
}
