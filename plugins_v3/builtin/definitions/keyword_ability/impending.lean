/-- "Impending N—[cost]" means "You may choose to pay [cost] rather than pay this spell's mana
cost," "If you chose to pay this permanent's impending cost, it enters with N time counters on
it," "As long as this permanent's impending cost was paid and it has a time counter on it, it's
not a creature," and "At the beginning of your end step, if this permanent's impending cost was
paid and it has a time counter on it, remove a time counter from it." [CR#702.176a] -/
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
                  .fresh]))),
      static
        (onlyWhile
          (characteristicChange thisPermanent
            [typeLine .loses
                { types := some [Semantics.CardType.creature] }])
          (and
            [keywordCostPaid impending.label thisPermanent,
              is thisPermanent
                (hasCounter (some timeCounter))])),
      at_ (beginningOfYour .endStep)
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
