/-- "Tiered" means "Choose one. As an additional cost to cast this spell, pay the cost associated
with that mode." [CR#702.183a] -/
keyword tiered : Ability := {
  lexical := { lemma := "tiered" },
  spelling := "tiered",
  body :=
    [static
        (modeChoice (setQuantity (oneThrough 1)))]
}
