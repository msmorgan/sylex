template tourachDiscardTrigger : Ability := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := whenever (does anOpponent (discard (a
    isCard)))
    (putCounters 1 p1p1Counter thisCreature)
}
