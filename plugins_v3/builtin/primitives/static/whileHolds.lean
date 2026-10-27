primitive whileHolds (condition : Condition) (does : Action) : StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.StaticSpec.whileHolds condition does
}
