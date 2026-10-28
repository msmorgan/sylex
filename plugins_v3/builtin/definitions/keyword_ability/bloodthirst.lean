/-- "Bloodthirst N" means "If an opponent was dealt damage this turn, this permanent enters with N
+1/+1 counters on it." [CR#702.54a] -/
keyword bloodthirst (amount : Amount) : Ability := {
  lexical := { lemma := "bloodthirst", parameters := [[Amount]] },
  spelling := "bloodthirst <amount>",
  body :=
    [static
        (replace
          (would (actor thisPermanent)
            enter)
          (doIf
            (was
              (dealDamage anyAmount anOpponent)
              (within soFarThisTurn))
            (withRiders
              [withCounters amount (printedKind p1p1Counter)
                  .fresh])))]
}
