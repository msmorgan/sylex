primitive previous (player : Option NounPhrase) : TurnRef := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.TurnRef.previous player
}
