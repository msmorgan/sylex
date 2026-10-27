/-- "an additional N": beyond what is otherwise allowed ("play an additional land"). -/
primitive additional (q : Quantity) : Quantity := {
  -- TODO(spelling): an additional N; wording varies with q, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.Quantity.additional q
}
