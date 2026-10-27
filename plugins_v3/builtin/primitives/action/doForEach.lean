primitive doForEach (group : NounPhrase) (body : Action) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.doForEach group body
}
