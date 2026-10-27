primitive counterKindOn (subject : NounPhrase) : Predicate := {
  spelling := "kind of counter on <subject>",
  body :=
    Semantics.Primitives.Predicate.counterKindOn subject
}
