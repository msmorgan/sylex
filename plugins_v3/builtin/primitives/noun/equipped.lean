/-- "equipped <w>" -/
primitive equipped (w : NounWord) : NounPhrase := {
  spelling := "equipped <w>",
  body :=
    Semantics.Primitives.NounPhrase.attachHost Semantics.AttachWord.equipped w
}
