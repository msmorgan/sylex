/-- Authoring primitive for `StaticSpec.conjunction`. -/
primitive conjunction (parts : List StaticSpec) (subject : Option NounPhrase := none) :
    StaticSpec := {
  body :=
    Semantics.Primitives.StaticSpec.conjunction subject parts
}
