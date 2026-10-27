template aPlayerCastTwoOrMoreLastTurn : Condition := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := compareAmt
    (aggregateOver .max isPlayer (eventCount (cast (a isSpell)) they lastTurn)) .atLeast 2
}
