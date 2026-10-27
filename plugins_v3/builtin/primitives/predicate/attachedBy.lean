/-- "enchanted by <by_>", "equipped by <by_>": the host of an attachment `by_` names. -/
primitive attachedBy (word : Option AttachWord) (by_ : NounPhrase) : Predicate := {
  -- TODO(spelling): enchanted by <by_>; wording varies with word, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.Predicate.attachment Semantics.AttachmentSide.host word (Option.some by_)
}
