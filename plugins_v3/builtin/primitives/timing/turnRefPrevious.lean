primitive turnRefPrevious (player : Option Semantics.NounPhrase) : Semantics.TurnRef := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.TurnRef.previous player
}
