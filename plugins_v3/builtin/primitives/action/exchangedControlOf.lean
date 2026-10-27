primitive exchangedControlOf (left : Semantics.NounPhrase) (right : Semantics.NounPhrase) : Semantics.Exchanged := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Exchanged.controlOf left right
}
