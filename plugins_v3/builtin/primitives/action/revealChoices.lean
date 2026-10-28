/-- Reveal the choices of the specified hidden sort. -/
primitive revealChoices (sort : HiddenSort) : Action := {
  body :=
    Semantics.Primitives.Action.revealChoices sort
}
