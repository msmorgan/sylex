/-- "Rebound" means "If this spell was cast from your hand, instead of putting it into your
graveyard as it resolves, exile it and, at the beginning of your next upkeep, you may cast this
card from exile without paying its mana cost." [CR#702.88a] -/
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
                    (beginningOf upkeep (some you)
                      (nextTime none))
                    (may
                      (cast this_ none
                        withoutPaying))]))))]
}
