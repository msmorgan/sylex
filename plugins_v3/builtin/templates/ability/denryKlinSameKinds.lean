template denryKlinSameKinds : Ability := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := whenever (does (a (and [isNontoken, and [isCreature, youControl]])) enter)
    (if_ := is thisCreature hasCounter)
    (doForEachKind .counterKind .counterKind
      (putCounters (statOf .boundCounter thisCreature) thatKind (that creature))
      (domain := thisCreature))
}
