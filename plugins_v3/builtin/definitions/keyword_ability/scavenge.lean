keyword scavenge (cost : Cost) : Ability := {
  lexical := { lemma := "scavenge", parameters := [[Cost]] },
  spelling := "scavenge <cost>",
  body :=
    [ activatedOnlyDuring
        (.compound
          [ cost,
            .perform (exileFrom thisCard yourGraveyard) ])
        (putCounters (powerOf (the (and [isCard, did you (exile which)])))
          p1p1Counter (target isCreature))
        asSorcery ]
}
