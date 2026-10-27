/-- "<keyword> or <keyword>", as an ability chosen among named keywords -/
primitive abilitiesAmong (keywords : List KeywordTerm) : ChoiceDomain := {
  -- TODO(spelling): Context-dependent wording: <keyword> or <keyword>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.ChoiceDomain.abilitiesAmong keywords
}
