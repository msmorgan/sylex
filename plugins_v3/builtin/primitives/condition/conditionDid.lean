primitive conditionDid (clause : Semantics.DidClause) : Semantics.Condition := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Condition.did clause
}
