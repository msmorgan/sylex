template forkedBolt : Action := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := dealDivided this_ 2 (targets (oneThrough 2) isAnyTarget)
}
