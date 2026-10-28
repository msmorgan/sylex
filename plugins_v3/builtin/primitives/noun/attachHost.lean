/-- The object described as enchanted, equipped, or fortified. -/
primitive attachHost (word : AttachWord) (head : NounWord) : NounPhrase := {
  spelling := "<word> <head>",
  body :=
    Semantics.Primitives.NounPhrase.attachHost word head
}
