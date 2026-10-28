/-- Authoring primitive for `StaticSpec.controlGrant`. -/
primitive controlGrant (player subject : NounPhrase) : StaticSpec := {
  spelling := "<player> controls <subject>",
  body :=
    Semantics.Primitives.StaticSpec.controlGrant player subject
}
