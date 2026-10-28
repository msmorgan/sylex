/-- Select the number of times the linked cost was paid. -/
primitive timesPaid (which : LinkSource) : LinkedFacet := {
  body := Semantics.Primitives.LinkedFacet.timesPaid which
}
