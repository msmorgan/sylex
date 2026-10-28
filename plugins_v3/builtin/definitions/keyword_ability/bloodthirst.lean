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
