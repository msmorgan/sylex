template pirDistributive : Ability := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := static (replace
            (would any
               (putCounters anyAmount anyKind
                  (a (and [isPermanent, controlledBy (youOr (a isTeammate))]))))
                      (instead (putCounters (plus thatMuch 1) thoseCounters (that permanent))))
}
