primitive moveRiderAttachedTo (host : Semantics.NounPhrase) : Semantics.MoveRider := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.MoveRider.attachedTo host
}
