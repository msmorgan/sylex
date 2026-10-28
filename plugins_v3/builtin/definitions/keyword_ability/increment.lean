/-- "Increment" means "Whenever you cast a spell, if this permanent is a creature and the amount of
mana spent to cast that spell is greater than this creature's power or this creature's
toughness, put a +1/+1 counter on this creature." [CR#702.191a] -/
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
                    (manaValueSpentToCast (that spell))
                    greater (powerOf thisCreature),
                  compareAmt
                    (manaValueSpentToCast (that spell))
                    greater (toughnessOf thisCreature)]]))]
}
