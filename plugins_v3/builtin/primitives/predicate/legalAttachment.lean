/-- Full-arity binding for `Predicate.legalAttachment`. -/
primitive legalAttachment (side : AttachmentSide) (counterpart : NounPhrase) : Predicate := {
  body := Semantics.Primitives.Predicate.legalAttachment side counterpart
}
