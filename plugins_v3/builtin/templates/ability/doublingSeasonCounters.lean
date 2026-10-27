template doublingSeasonCounters : Ability := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := static (replace
    (causedBy byAnEffect
      (would any (putCounters anyAmount anyKind (a (and [isPermanent, youControl])))))
    (instead (putCounters (times 2 thatMuch) thoseCounters (that permanent))))
}
