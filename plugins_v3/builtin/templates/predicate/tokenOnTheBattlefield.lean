template tokenOnTheBattlefield : Predicate := {
  spelling := "token on the battlefield",
  body :=
    Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isToken, Semantics.Templates.Predicate.isPermanent]
}
