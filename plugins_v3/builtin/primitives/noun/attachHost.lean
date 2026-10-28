/-- The object described as enchanted, equipped, or fortified. -/
primitive attachHost (word : AttachWord) (head : NounWord) : NounPhrase := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.NounPhrase.attachHost word head
}
