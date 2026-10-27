template castSmallCreatureFromTopOnceEachTurn : StaticSpec := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := can you
      (cast (counted (exactly 1) (and [isSpell, isCreature, hasPower .atMost 2]))
         (from_ := onTop)) (timing := eachTurn)
}
