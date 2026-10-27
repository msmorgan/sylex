template cardOnTheStack : Predicate := {
  spelling := "card on the stack",
  body :=
    Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isCard, Semantics.Templates.Predicate.isSpell]
}
