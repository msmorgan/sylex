/-- "Bestow [cost]" means "As you cast this spell, you may choose to cast it bestowed. If you do,
you pay [cost] rather than its mana cost." [CR#702.103a] -/
keyword bestow (cost : Cost) : Ability := {
  lexical := { lemma := "bestow", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): bestow [mana cost] / bestow—[verbal cost]; separator and terminal period
  -- depend on the Cost structure.
  body :=
    [static (altCost this_ (some cost))]
}
