/-- "target <subject>": the act of targeting [CR#115.1]. -/
-- [CR#115.1]
primitive Action.target (subject : NounPhrase) : Action := {
  spelling := "target <subject>",
  body :=
    Semantics.Primitives.Action.target subject
}
