keyword rebound : Ability := {
  lexical := { lemma := "rebound" },
  spelling := "rebound",
  body :=
    [static
        (replace
          (causedBy
            (byEvent
              (does Subject.any
                (resolve thisSpell)))
            (would Subject.any
              (putInto thisSpell graveyard
                (some stack))))
          (doIf
            (is thisSpell
              (did Subject.any
                (cast which (some yourHand))
                (some thisObject)))
            (instead
              (sequentially
                [exile thisSpell,
                  delay
                    (beginningOf Semantics.TurnPart.upkeep (some you)
                      (nextTime none))
                    (may
                      (cast this_ none
                        withoutPaying))]))))]
}
