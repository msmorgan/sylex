/-- "<performer> explores" (`Definition`, [CR#701.44a]). -/
action explore : Action := {
  lexical := { verbs := [{ lemma := "explore", frames := [Intransitive, Monotransitive] }] },
  permanent := performs,
  spelling := "explore",
  body :=
    instruct (controllerOf theActor)
      (sequentially
        [reveal
            (slice .top 1 (libraryOf actor)),
          doIf
            (is (that .card) isLand)
            (putInto (that .card)
              (zone .hand
                (some (ownerOf (that .card)))))
            (some
              (sequentially
                [putCounters 1 (printedKind p1p1Counter)
                    theActor,
                  may
                    (putInto (that .card)
                      graveyard)]))])
}
