primitive establish (spec : StaticSpec) (duration : Option Duration := none) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.establish spec duration
}
