primitive copyChange (subject source : NounPhrase) (exceptions : List CopyExcept := []) :
    StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.StaticSpec.copyChange subject source exceptions
}
