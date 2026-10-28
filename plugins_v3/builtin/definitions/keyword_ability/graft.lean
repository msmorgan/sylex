/-- "Graft N" means "This permanent enters with N +1/+1 counters on it" and "Whenever another
creature enters, if this permanent has a +1/+1 counter on it, you may move a +1/+1 counter from
this permanent onto that creature." [CR#702.58a] -/
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
