/-- A token copy of the source with the specified copy exceptions. -/
primitive copyOf (source : NounPhrase) (exceptions : List CopyExcept := []) : TokenSpec := {
  body :=
    Semantics.Primitives.TokenSpec.copyOf source exceptions
}
