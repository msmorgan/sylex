template castCreatureSpellsFromTop : StaticSpec := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := can you (cast (bare (and [isSpell, isCreature])) (from_ := onTop))
}
