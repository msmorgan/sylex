/-- "Overload [cost]" means "You may choose to pay [cost] rather than pay this spell's mana cost"
and "If you chose to pay this spell's overload cost, change its text by replacing all instances
of the word 'target' with the word 'each.'" [CR#702.96a] -/
keyword overload (cost : Cost) : Ability := {
  lexical := { lemma := "overload", parameters := [[Cost]] },
  spelling := "overload <cost>",
  body :=
    [static (altCost this_ (some cost)),
      static
        (onlyWhile
          (textChange thisSpell targetToEach)
          (keywordCostPaid overload.label thisSpell))]
}
