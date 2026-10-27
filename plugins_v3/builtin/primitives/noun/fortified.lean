/-- "fortified <w>" -/
primitive fortified (w : NounWord) : NounPhrase := {
  spelling := "fortified <w>",
  body :=
    Semantics.Primitives.NounPhrase.attachHost Semantics.AttachWord.fortified w
}
