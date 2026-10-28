/-- Authoring primitive for `StaticSpec.visibility`. -/
primitive visibility (verb : ExposeVerb) (player : NounPhrase) (what : VisibleThing) :
    StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.StaticSpec.visibility verb player what
}
