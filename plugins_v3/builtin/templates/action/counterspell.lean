template counterspell : Action := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := counter (target isSpell)
}
