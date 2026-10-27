primitive addedCost (cost : Cost) (offered : Bool) (times : PayTimes := PayTimes.once) :
    StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.StaticSpec.addedCost cost offered times
}
