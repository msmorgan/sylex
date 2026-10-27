keyword graft (amount : Amount) : Ability := {
  lexical := { lemma := "graft", parameters := [[Amount]] },
  spelling := "graft <amount>",
  body :=
    [ static (entersWithCounters thisPermanent amount p1p1Counter),
      whenever
        (does (a (and [isCreature, isOther])) enter)
        (if_ := is thisPermanent (.hasCounters (some p1p1Counter)))
        (may (moveCounters 1 thisPermanent (that creature) p1p1Counter)) ]
}
