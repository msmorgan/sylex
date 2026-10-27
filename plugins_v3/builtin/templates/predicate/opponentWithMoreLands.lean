template opponentWithMoreLands : Predicate := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := compareOver isOpponent (countOf (and [isLand, controlledBy they])) .greater
    (countOf (and [isLand, youControl]))
}
