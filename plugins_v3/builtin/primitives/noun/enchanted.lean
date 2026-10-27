/-- "enchanted <w>" -/
primitive enchanted (w : NounWord) : NounPhrase := {
  spelling := "enchanted <w>",
  body :=
    Semantics.Primitives.NounPhrase.attachHost Semantics.AttachWord.enchanted w
}
