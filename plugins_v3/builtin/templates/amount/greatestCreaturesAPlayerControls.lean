template greatestCreaturesAPlayerControls : Amount := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := aggregateOver .max isPlayer (countOf (and [isCreature, controlledBy they]))
}
