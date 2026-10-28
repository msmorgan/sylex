keyword graft (amount : Amount) : Ability := {
  lexical := { lemma := "graft", parameters := [[Amount]] },
  spelling := "graft <amount>",
  body :=
    [static
        (entersWithCounters thisPermanent amount p1p1Counter),
      whenever
        (does
          (actor
            (a
              (and [isCreature, isOther])))
          enter)
        (may
          (moveCounters 1 thisPermanent
            (that creature)
            (some (printedKind p1p1Counter))))
        [] [] none none none
        (some
          (is thisPermanent
            (hasCounter (some p1p1Counter))))]
}
