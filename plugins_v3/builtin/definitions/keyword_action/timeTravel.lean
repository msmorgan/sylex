/-- "time travel" [CR#701.56a]: any number of permanents the performer controls with time counters
and/or suspended cards they own in exile with time counters, a suspended card being one in exile
that has suspend and a time counter [CR#702.62b]; for each, put or remove a time counter
(`Definition`). -/
action timeTravel : Action := {
  lexical := { verbs := [{ lemma := "time travel", frames := [Intransitive] }] },
  spelling := "time travel",
  body :=
    sequentially
        [ choose
            (counted anyNumber
              (andOr
                [ and [isPermanent, actorControls, .hasCounters timeCounter],
                  and
                    [ .isCard, .in_ exileZone, ownedBy .actor,
                      hasKeyword suspend, .hasCounters timeCounter ] ])),
          .doForEach them
            (chooseOne (putCounters 1 timeCounter it)
              (removeCounters (oneThrough 1) timeCounter it)) ]
}
