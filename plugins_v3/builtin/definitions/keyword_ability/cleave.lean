/-- "Cleave [cost]" means "You may cast this spell by paying [cost] rather than paying its mana
cost" and "If this spell's cleave cost was paid, change its text by removing all text found
within square brackets in the spell's rules text." [CR#702.148a] -/
keyword cleave (cost : Cost) : Ability := {
  lexical := { lemma := "cleave", parameters := [[Cost]], bracketed_text_removable := true },
  spelling := "cleave <cost>",
  body :=
    [static (altCost this_ (some cost)),
      static
        (onlyWhile
          (textChange thisSpell removeBracketed)
          (keywordCostPaid cleave.label thisSpell))]
}
