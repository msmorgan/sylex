template isNontoken : Predicate := {
  spelling := "nontoken",
  body :=
    Semantics.Templates.Predicate.not Semantics.Templates.Predicate.isToken
}
