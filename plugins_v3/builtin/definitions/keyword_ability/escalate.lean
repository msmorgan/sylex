/-- "Escalate [cost]" means "For each mode you choose beyond the first as you cast this spell, you
pay an additional [cost]" [CR#702.120a] -/
keyword escalate (cost : Cost) : Ability := {
  lexical := { lemma := "escalate", parameters := [[Cost], [DashNonManaCost]] },
  spelling := "escalate <cost>",
  body :=
    [static (modeChoice (extraModesCost cost))]
}
