/-- "Spree" means "Choose one or more modes. As an additional cost to cast this spell, pay the costs
associated with those modes." [CR#702.172a] -/
keyword spree : Ability := {
  lexical := { lemma := "spree" },
  spelling := "spree",
  body :=
    [static
        (modeChoice (setQuantity (orMore 1)))]
}
