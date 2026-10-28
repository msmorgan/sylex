primitive mostRecent (player : NounPhrase) : TurnRef := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.TurnRef.mostRecent player
}
