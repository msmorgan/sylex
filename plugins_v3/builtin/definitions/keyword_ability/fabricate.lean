/-- "Fabricate N" means "When this permanent enters, you may put N +1/+1 counters on it. If you
don't, create N 1/1 colorless Servo artifact creature tokens." [CR#702.123a] -/
keyword fabricate (amount : Amount) : Ability := {
  lexical := { lemma := "fabricate", parameters := [[Amount]] },
  spelling := "fabricate <amount>",
  body :=
    [ when (does thisPermanent enter)
        (mayOrElse (putCounters amount p1p1Counter thisPermanent)
          (create amount (artifactCreatureToken 1 1 [] [servo]))) ]
}
