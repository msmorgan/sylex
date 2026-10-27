/-- "that could enchant <host>": an Aura that could legally be attached to `host` [CR#701.3a]. -/
-- [CR#701.3a]
primitive couldEnchant (host : NounPhrase) : Predicate := {
  spelling := "that could enchant <host>",
  body :=
    Semantics.Primitives.Predicate.legalAttachment Semantics.AttachmentSide.attachment host
}
