/-- "{A/B}" -/
template hybridPip (left right : Color) : ManaSymbol := {
  -- TODO(spelling): Context-dependent wording: {A/B}; string substitution cannot express its omitted or inflected parts.
  body :=
    .hybrid (.specific (.of left)) right
}
