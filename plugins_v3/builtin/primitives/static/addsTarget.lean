/-- Authoring primitive for `StaticSpec.addsTarget`. -/
primitive addsTarget (subject : NounPhrase) (description : Predicate) : StaticSpec := {
  body :=
    Semantics.Primitives.StaticSpec.addsTarget subject description
}
