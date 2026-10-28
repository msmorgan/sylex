/-- "Awaken N—[cost]" means "You may pay [cost] rather than pay this spell's mana cost as you cast
this spell" and "If this spell's awaken cost was paid, put N +1/+1 counters on target land you
control. That land becomes a 0/0 Elemental creature with haste. It's still a land."
[CR#702.113a] -/
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
