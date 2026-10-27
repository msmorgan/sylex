/-- "This Aura/Equipment/Fortification is attached": the attachment relation the engine's
attach-permission model keeps [CR#704.5m,704.5n]. -/
template isAttached (subject : NounPhrase) : Condition := {

  -- TODO(spelling): <subject> is/are attached; string substitution cannot choose subject-verb
  -- agreement.
  body :=
    Semantics.Templates.Condition.is subject
      (Semantics.Templates.Predicate.predicateAttachment Semantics.AttachmentSide.attachment Option.none Option.none)
}
