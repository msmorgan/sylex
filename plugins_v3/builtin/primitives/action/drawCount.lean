/-- "draw up to N cards", "can't draw more than one card": a draw bounded by a Quantity. -/
primitive drawCount (quantity : Quantity) : Action := {
  -- TODO(spelling): draw up to N cards; wording varies with quantity, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.Action.draw quantity
}
