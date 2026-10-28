/-- "<keyword> or <keyword>", as an ability chosen among named keywords -/
primitive abilitiesAmong (keywords : List KeywordTerm) : ChoiceDomain := {
  body :=
    Semantics.Primitives.ChoiceDomain.abilitiesAmong keywords
}
