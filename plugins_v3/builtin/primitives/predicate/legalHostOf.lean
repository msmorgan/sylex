/-- "a legal object or player for <attachment> to enchant" [CR#303.4f,303.4g] -/
-- [CR#303.4f,303.4g]
primitive legalHostOf (attachment : NounPhrase) : Predicate := {
  spelling := "a legal object or player for <attachment> to enchant",
  body :=
    Semantics.Primitives.Predicate.legalAttachment Semantics.AttachmentSide.host attachment
}
