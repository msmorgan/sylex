keyword vanishing (amount : Amount) : Ability := {
  lexical := { lemma := "vanishing", parameters := [[], [Amount]] },
  spelling := "vanishing <amount>",
  body :=
    [ static (entersWithCounters thisPermanent amount timeCounter),
      at_ (beginningOfYour .upkeep)
        (if_ := is thisPermanent (.hasCounters (some timeCounter)))
        (removeCounters (oneThrough 1) timeCounter thisPermanent),
      when (does any (removeCounters theLast timeCounter thisPermanent))
        (sacrifice thisPermanent) ]
}
