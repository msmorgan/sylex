/-- "enchanted", "equipped", "fortified": the host of an attachment of that kind, or of any
attachment when `word` is none. -/
template isAttached (word : Option AttachWord := none) : Predicate := {
  -- TODO(spelling): <subject> is/are attached; string substitution cannot choose subject-verb
  -- agreement.
  body := attachment .host none word
}
