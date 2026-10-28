primitive cardsAcross (left : NounPhrase) (right : NounPhrase) : Exchanged := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Exchanged.cardsAcross left right
}
