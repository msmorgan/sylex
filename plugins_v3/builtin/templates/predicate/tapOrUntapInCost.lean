/-- "with the tap symbol or the untap symbol in its activation cost" [CR#302.6,107.5,107.6]. -/
-- [CR#302.6,107.5,107.6]
template tapOrUntapInCost : Predicate := {
  spelling := "with the tap symbol or the untap symbol in its activation cost",
  body :=
    Semantics.Templates.Predicate.or
      [Semantics.Templates.Predicate.activationCostHas Semantics.Templates.Cost.tapSymbol,
        Semantics.Templates.Predicate.activationCostHas Semantics.Templates.Cost.untapSymbol]
}
