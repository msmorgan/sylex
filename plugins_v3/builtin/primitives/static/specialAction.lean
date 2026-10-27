primitive specialAction (when : Timing) (cost : Cost) (does : Action) : StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.StaticSpec.specialAction Option.none (Option.some when) cost does
}
