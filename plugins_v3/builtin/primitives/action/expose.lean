/-- Reveal or look at the specified cards or zone. -/
primitive expose (verb : ExposeVerb) (exposed : Exposed) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.expose verb exposed
}
