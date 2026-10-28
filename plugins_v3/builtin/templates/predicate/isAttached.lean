/-- "enchanted", "equipped", "fortified": the host of an attachment of that kind, or of any
attachment when `word` is none. -/
template isAttached (word : Option AttachWord := none) : Predicate := {
  -- TODO(spelling): Include or omit the clauses supplied by word.
  body := attachment .host none word
}
