keyword increment : Ability := {
  lexical := { lemma := "increment" },
  spelling := "increment",
  body :=
    [whenever
        (does (actor you)
          (cast (a isSpell)))
        (putCounters 1 (printedKind p1p1Counter)
          thisCreature)
        [] [] none none none
        (some
          (and
            [is thisPermanent isCreature,
              or
                [compareAmt
                    (manaValueSpentToCast (that Semantics.NounWord.spell))
                    Semantics.Comparator.greater (powerOf thisCreature),
                  compareAmt
                    (manaValueSpentToCast (that Semantics.NounWord.spell))
                    Semantics.Comparator.greater (toughnessOf thisCreature)]]))]
}
