/-- Authoring primitive for `StaticSpec.partSkip`. -/
primitive partSkip (player : NounPhrase) (part : TurnPart) : StaticSpec := {
  spelling := "<player> skips <part>",
  body :=
    Semantics.Primitives.StaticSpec.partSkip player part
}
