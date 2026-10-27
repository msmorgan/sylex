/-- "attached to <host>" -/
primitive attachedTo (host : NounPhrase) : Predicate := {
  spelling := "attached to <host>",
  body :=
    Semantics.Primitives.Predicate.attachment Semantics.AttachmentSide.attachment Option.none (Option.some host)
}
