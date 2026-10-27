keyword fading (amount : Amount) : Ability := {
  lexical := { lemma := "fading", parameters := [[Amount]] },
  spelling := "fading <amount>",
  body :=
    [ static (entersWithCounters thisPermanent amount fadeCounter),
      at_ (beginningOfYour .upkeep)
        (doIfDone (removeCounters (oneThrough 1) fadeCounter thisPermanent) none
          (sacrifice thisPermanent)) ]
}
