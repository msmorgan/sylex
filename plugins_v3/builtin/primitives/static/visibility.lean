/-- Authoring primitive for `StaticSpec.visibility`. -/
primitive visibility (verb : ExposeVerb) (player : NounPhrase) (what : VisibleThing) :
    StaticSpec := {
  body :=
    Semantics.Primitives.StaticSpec.visibility verb player what
}
