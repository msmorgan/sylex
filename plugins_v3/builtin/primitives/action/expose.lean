/-- Reveal or look at the specified cards or zone. -/
primitive expose (verb : ExposeVerb) (exposed : Exposed) : Action := {
  body :=
    Semantics.Primitives.Action.expose verb exposed
}
