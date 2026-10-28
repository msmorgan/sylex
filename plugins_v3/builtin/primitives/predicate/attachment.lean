/-- Authoring primitive for `Predicate.attachment`. -/
primitive attachment (side : AttachmentSide)
    (counterpart : Option NounPhrase := none) (word : Option AttachWord := none) : Predicate := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Predicate.attachment side word counterpart
}
