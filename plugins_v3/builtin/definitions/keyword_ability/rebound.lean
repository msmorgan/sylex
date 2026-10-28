keyword rebound : Ability := {
  lexical := { lemma := "rebound" },
  spelling := "rebound",
  body :=
    [static
        (replace
          (causedBy
            (byEvent
              (does any
                (resolve thisSpell)))
            (would any
              (putInto thisSpell graveyard
                (some stack))))
          (doIf
            (is thisSpell
              (did any
                (cast which (some yourHand))
                (some thisObject)))
            (instead
              (sequentially
                [exile thisSpell,
                  delay
                    (beginningOf .upkeep (some you)
                      (nextTime none))
                    (may
                      (cast this_ none
                        withoutPaying))]))))]
}
