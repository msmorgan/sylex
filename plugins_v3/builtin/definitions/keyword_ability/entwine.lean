/-- "Entwine [cost]" means "You may choose all modes of this spell instead of just the number
specified. If you do, you pay an additional [cost]." [CR#702.42a] -/
keyword entwine (cost : Cost) : Ability := {
  lexical := { lemma := "entwine", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): entwine [mana cost] / entwine—[verbal cost]; separator and terminal period
  -- depend on the Cost structure.
  body :=
    [static
        (modeChoice (allInstead (some cost) none))]
}
