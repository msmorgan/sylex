keyword backup (amount : Amount) : Ability := {
  lexical := { lemma := "backup", parameters := [[Amount]] },
  spelling := "backup <amount>",
  body :=
    [when
        (does (actor thisCreature) enter)
        (sequentially
          [putCounters amount (printedKind p1p1Counter)
              (target isCreature),
            doIf
              (is (that creature)
                (and [isCreature, isOther]))
              (establishThroughout
                (abilityGrantSelected (that creature)
                  thisCreature
                  (printedAfter backup.label))
                untilEndOfTurn)])]
}
