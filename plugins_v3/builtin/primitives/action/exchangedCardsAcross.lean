primitive exchangedCardsAcross (left : Semantics.NounPhrase) (right : Semantics.NounPhrase) : Semantics.Exchanged := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Exchanged.cardsAcross left right
}
