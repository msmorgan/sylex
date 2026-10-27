/-- "<subject> commits a crime" [CR#700.13] -/
-- [CR#700.13]
primitive commitCrime : Action := {
  -- TODO(spelling): Context-dependent wording: <subject> commits a crime; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Action.commitCrime
}
