/-- "Bestow [cost]" means "As you cast this spell, you may choose to cast it bestowed. If you do,
you pay [cost] rather than its mana cost." [CR#702.103a] -/
keyword bestow (cost : Cost) : Ability := {
  lexical := { lemma := "bestow", parameters := [[Cost]] },
  spelling := "bestow <cost>",
  body :=
    [static (altCost this_ (some cost))]
}
