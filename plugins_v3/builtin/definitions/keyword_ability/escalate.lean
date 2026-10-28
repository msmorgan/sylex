/-- "Escalate [cost]" means "For each mode you choose beyond the first as you cast this spell, you
pay an additional [cost]" [CR#702.120a] -/
keyword escalate (cost : Cost) : Ability := {
  lexical := { lemma := "escalate", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): escalate [mana cost] / escalate—[verbal cost]; separator and terminal period
  -- depend on the Cost structure.
  body :=
    [static (modeChoice (extraModesCost cost))]
}
