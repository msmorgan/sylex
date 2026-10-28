/-- "<action> unless you pay <whose>'s <keyword> cost": `doUnless` whose payment is of the keyword
cost of `whose`, as cumulative upkeep's and echo's are [CR#702.24a,702.30a], so a trigger on paying
that cost and mana spendable only on it read this payment. -/
-- [CR#702.24a,702.30a]
template doUnlessKeywordCost (action : Action) (cost : Cost) (whose : NounPhrase)
    (keyword : KeywordLabel) : Action := {
  spelling := "<action> unless you pay <whose>'s <keyword> cost",
  body :=
    withContinuation .optional
      (actionPay (Semantics.Cost.asPaid cost) .once .paid
        (some whose) (some keyword))
      none (some action)
}
