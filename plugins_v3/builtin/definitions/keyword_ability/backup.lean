keyword backup (amount : Amount) : Ability := {
  lexical := { lemma := "backup", parameters := [[Amount]] },
  spelling := "backup <amount>",
  body :=
    [ when (does thisCreature enter)
        (sequentially
          [ putCounters amount p1p1Counter (target isCreature),
            doIf (is (that creature) (and [isCreature, isOther]))
              (establishThroughout
                (Templates.abilityGrantSelected (that creature) thisCreature
                  (.printedAfter backup))
                untilEndOfTurn)
              none ]) ]
}
