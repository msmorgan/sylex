primitive hasKeyword (kw : KeywordTerm) : Predicate := {
  spelling := "with <kw>",
  body :=
    Semantics.Primitives.Predicate.hasKeyword kw
}
