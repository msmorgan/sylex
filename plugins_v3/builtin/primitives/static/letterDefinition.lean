/-- Authoring primitive for `StaticSpec.letterDefinition`. -/
primitive letterDefinition (letter : Letter) (amount : Amount) : StaticSpec := {
  spelling := "<letter> is <amount>",
  body :=
    Semantics.Primitives.StaticSpec.letterDefinition letter amount
}
