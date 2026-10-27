template playLandsAndCastSpellsFromTop : StaticSpec := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := conjunction [playLandsFromTop, castFromTop isSpell]
}
