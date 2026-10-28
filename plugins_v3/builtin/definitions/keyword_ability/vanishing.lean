/-- "Vanishing N" means "This permanent enters with N time counters on it," "At the beginning of
your upkeep, if this permanent has a time counter on it, remove a time counter from it," and
"When the last time counter is removed from this permanent, sacrifice it." [CR#702.63a] -/
keyword vanishing (amount : Amount) : Ability := {
  lexical := { lemma := "vanishing", parameters := [[], [Amount]] },
  spelling := "vanishing <amount>",
  body :=
    [static
        (entersWithCounters thisPermanent amount timeCounter),
      at_ (beginningOfYour .upkeep)
        (removeCounters (oneThrough 1)
          (some (printedKind timeCounter))
          thisPermanent)
        [] [] none none none
        (some
          (is thisPermanent
            (hasCounter (some timeCounter)))),
      when
        (does any
          (removeCounters theLast
            (some (printedKind timeCounter))
            thisPermanent))
        (sacrifice thisPermanent)]
}
