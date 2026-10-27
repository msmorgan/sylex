/-- "a token that's …", entering with `riders` -/
primitive CreationSpec.token (spec : TokenSpec) (riders : List MoveRider) : CreationSpec := {
  -- TODO(spelling): Context-dependent wording: a token that's …; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.CreationSpec.token spec riders
}
