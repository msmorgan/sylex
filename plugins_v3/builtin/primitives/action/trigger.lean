/-- "<ability> triggers": the act a trigger prohibition names. -/
primitive trigger : Action := {
  -- TODO(spelling): Context-dependent wording: <ability> triggers; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Action.trigger
}
