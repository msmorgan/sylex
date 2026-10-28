/-- "Fading N" means "This permanent enters with N fade counters on it" and "At the beginning of
your upkeep, remove a fade counter from this permanent. If you can't, sacrifice the permanent."
[CR#702.32a] -/
keyword fading (amount : Amount) : Ability := {
  lexical := { lemma := "fading", parameters := [[Amount]] },
  spelling := "fading <amount>",
  body :=
    [ static (entersWithCounters thisPermanent amount fadeCounter),
      at_ (beginningOfYour upkeep)
        (doIfDone (removeCounters (oneThrough 1) fadeCounter thisPermanent) none
          (sacrifice thisPermanent)) ]
}
