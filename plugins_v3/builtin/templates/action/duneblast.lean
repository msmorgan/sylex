template duneblast : Action := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := sequentially [choose (counted (upTo 1) isCreature), destroy theRest]
}
