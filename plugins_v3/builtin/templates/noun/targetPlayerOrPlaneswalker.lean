template targetPlayerOrPlaneswalker : NounPhrase := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := target (or [isPlaneswalker, isPlayer])
}
