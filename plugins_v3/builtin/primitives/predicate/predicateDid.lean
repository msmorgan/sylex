primitive predicateDid (clause : Semantics.DidClause) : Semantics.Predicate := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Predicate.did clause
}
