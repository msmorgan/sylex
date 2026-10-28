/-- Authoring primitive for `Predicate.attachment`. -/
primitive attachment (side : AttachmentSide)
    (counterpart : Option NounPhrase := none) (word : Option AttachWord := none) : Predicate := {
  body :=
    Semantics.Primitives.Predicate.attachment side word counterpart
}
