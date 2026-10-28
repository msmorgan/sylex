/-- An Action performed as the Event happens, without using the stack [CR#113.9]. -/
primitive asEvent (event : GameEvent) (does : Action) : StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.StaticSpec.asEvent event does
}
