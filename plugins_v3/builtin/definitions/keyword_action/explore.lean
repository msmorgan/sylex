/-- "<performer> explores" (`Definition`, [CR#701.44a]). -/
action explore : Action := {
  lexical := { verbs := [{ lemma := "explore", frames := [Intransitive, Monotransitive] }] },
  permanent := performs,
  spelling := "explore",
  body :=
    instruct (controllerOf theActor)
      (sequentially
        [reveal
            (slice Semantics.PileEnd.top 1 (libraryOf actor)),
          doIf
            (is (that Semantics.NounWord.card) isLand)
            (putInto (that Semantics.NounWord.card)
              (zone Semantics.Zone.hand
                (some (ownerOf (that Semantics.NounWord.card)))))
            (some
              (sequentially
                [putCounters 1 (printedKind p1p1Counter)
                    theActor,
                  may
                    (putInto (that Semantics.NounWord.card)
                      graveyard)]))])
}
