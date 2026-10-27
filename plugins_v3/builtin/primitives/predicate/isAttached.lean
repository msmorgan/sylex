/-- "enchanted", "equipped", "fortified": the host of an attachment of that kind, or of any
attachment when `word` is none. -/
primitive isAttached (word : Option AttachWord := none) : Predicate := {
  -- TODO(spelling): <subject> is/are attached; string substitution cannot choose subject-verb
  -- agreement.
  body :=
    Semantics.Primitives.Predicate.attachment Semantics.AttachmentSide.host word Option.none
}
