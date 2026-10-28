/-- Change the attachment between the subject and its host. -/
primitive attachment (move : AttachMove) (subject : NounPhrase)
    (host : Option NounPhrase := none) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.attachment move subject host
}
