keyword impending (amount : Amount) (cost : Cost) : Ability := {
  lexical := { lemma := "impending", parameters := [[Amount, DashCost]] },
  spelling := "impending <amount>—<cost>",
  body :=
    [ static (altCost this_ (some cost)),
      static
        (replace (would thisPermanent enter)
          (doIf (keywordCostPaid impending thisPermanent)
            (withRiders [.withCounters amount timeCounter .fresh]))),
      static
        (onlyWhile
          (.characteristicChange thisPermanent [.typeLine .loses { types := some [.creature] }])
          (and
            [ keywordCostPaid impending thisPermanent,
              is thisPermanent (.hasCounters timeCounter) ])),
      at_ (beginningOfYour .endStep)
        (if_ := some (and
          [ keywordCostPaid impending thisPermanent,
            is thisPermanent (.hasCounters timeCounter) ]))
        (removeCounters (oneThrough 1) timeCounter thisPermanent) ]
}
