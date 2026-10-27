template woeleecher : Ability := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := activated (compound [[pip .white], tapSymbol])
    (doIfDone
      (removeCounters (exactly 1) m1m1Counter (target isCreature))
      (gainLife 2))
}
