/-- "Entwine [cost]" means "You may choose all modes of this spell instead of just the number
specified. If you do, you pay an additional [cost]." [CR#702.42a] -/
keyword entwine (cost : Cost) : Ability := {
  lexical := { lemma := "entwine", parameters := [[Cost]] },
  spelling := "entwine <cost>",
  body :=
    [static
        (modeChoice (allInstead (some cost) none))]
}
