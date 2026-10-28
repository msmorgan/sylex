/-- "<event> causes …" -/
primitive byEvent (event : GameEvent) : Causing := {
  body :=
    Semantics.Primitives.Causing.event event
}
