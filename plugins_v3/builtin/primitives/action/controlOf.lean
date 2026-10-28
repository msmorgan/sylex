primitive controlOf (left : NounPhrase) (right : NounPhrase) : Exchanged := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Exchanged.controlOf left right
}
