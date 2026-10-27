keyword bloodthirst (amount : Amount) : Ability := {
  lexical := { lemma := "bloodthirst", parameters := [[Amount]] },
  spelling := "bloodthirst <amount>",
  body :=
    [ static
        (replace (would thisPermanent enter)
          (doIf (Condition.was (dealDamage anyAmount anOpponent) soFarThisTurn)
            (withRiders [.withCounters amount p1p1Counter .fresh]))) ]
}
