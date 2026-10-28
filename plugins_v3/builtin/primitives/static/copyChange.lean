/-- Authoring primitive for `StaticSpec.copyChange`. -/
primitive copyChange (subject source : NounPhrase) (exceptions : List CopyExcept := []) :
    StaticSpec := {
  body :=
    Semantics.Primitives.StaticSpec.copyChange subject source exceptions
}
