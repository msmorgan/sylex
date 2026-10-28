keyword gift (promise : Action) : Ability := {
  lexical := { lemma := "gift", parameters := [[Quality (None)]], nouns := ["gift"] },
  -- TODO(spelling): Gift a card / gift a tapped Fish; the promised Action must select a licensed
  -- noun phrase, not print an action clause.
  body :=
    [static
        (addedCost (perform (choose anOpponent))
          true .once),
      byCardClass
        (when
          (does (actor thisPermanent) enter)
          (instruct (the isChosenPlayer) promise) [] []
          none none none
          (some (keywordCostPaid gift.label this_)))
        (spell
          (doIf
            (keywordCostPaid gift.label this_)
            (instruct (the isChosenPlayer) promise)))]
}
