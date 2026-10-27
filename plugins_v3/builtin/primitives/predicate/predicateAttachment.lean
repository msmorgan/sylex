primitive predicateAttachment (side : Semantics.AttachmentSide) (word : Option Semantics.AttachWord) (counterpart : Option Semantics.NounPhrase) : Semantics.Predicate := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Predicate.attachment side word counterpart
}
