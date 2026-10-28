/-- Select the number of times the linked cost was paid. -/
primitive timesPaid (which : LinkSource) : LinkedFacet := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.LinkedFacet.timesPaid which
}
