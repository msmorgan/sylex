/-- "Reinforce N—[cost]" means "[Cost], Discard this card: Put N +1/+1 counters on target creature."
[CR#702.77a] -/
keyword reinforce (amount : Amount) (cost : Cost) : Ability := {
  lexical := { lemma := "reinforce", parameters := [[Amount, DashCost]] },
  spelling := "reinforce <amount>—<cost>",
  body :=
    [activated
        (compound [cost, perform (discard this_)])
        (putCounters amount (printedKind p1p1Counter)
          (target isCreature))]
}
