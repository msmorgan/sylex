template theKeyword (label : KeywordLabel) : KeywordTerm := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.KeywordTerm.the label
}
