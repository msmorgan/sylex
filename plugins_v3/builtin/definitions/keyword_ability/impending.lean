keyword impending (amount : Amount) (cost : Cost) : Ability := {
  lexical := { lemma := "impending", parameters := [[Amount, DashCost]] },
  spelling := "impending <amount>—<cost>",
  body :=
    [static (altCost this_ (some cost)),
      static
        (replace
          (would (actor thisPermanent)
            enter)
          (doIf
            (keywordCostPaid impending.label thisPermanent)
            (withRiders
              [withCounters amount (printedKind timeCounter)
                  Semantics.EntryCounterMark.fresh]))),
      static
        (onlyWhile
          (characteristicChange thisPermanent
            [characteristicEditTypeLine Semantics.QualityOp.loses
                { types := some [Semantics.CardType.creature] }])
          (and
            [keywordCostPaid impending.label thisPermanent,
              is thisPermanent
                (hasCounter (some timeCounter))])),
      at_ (beginningOfYour Semantics.TurnPart.endStep)
        (removeCounters (oneThrough 1)
          (some (printedKind timeCounter))
          thisPermanent)
        [] [] none none none
        (some
          (and
            [keywordCostPaid impending.label thisPermanent,
              is thisPermanent
                (hasCounter (some timeCounter))]))]
}
