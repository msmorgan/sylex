template garrukRelentlessFlip : Ability := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := when
    (stateHolds
      (is thisPlaneswalker (compare .atMost 2 (axes := [.counter loyaltyCounter]))))
    (transform thisPlaneswalker)
}
