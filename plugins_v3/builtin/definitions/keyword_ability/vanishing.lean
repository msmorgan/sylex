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
        (does Templates.Subject.any
          (removeCounters theLast
            (some (printedKind timeCounter))
            thisPermanent))
        (sacrifice thisPermanent)]
}
