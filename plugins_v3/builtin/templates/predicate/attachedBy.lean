/-- "enchanted by <by_>", "equipped by <by_>": the host of an attachment `by_` names. -/
template attachedBy (word : Option AttachWord) (by_ : NounPhrase) : Predicate := {
  -- TODO(spelling): Include or omit the clauses supplied by word.
  body := attachment .host (some by_) word
}
