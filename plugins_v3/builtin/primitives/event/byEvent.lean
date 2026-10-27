/-- "<event> causes …" -/
primitive byEvent (event : GameEvent) : Causing := {
  -- TODO(spelling): Context-dependent wording: <event> causes …; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Causing.event event
}
