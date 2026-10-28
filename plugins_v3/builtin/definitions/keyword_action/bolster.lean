/-- Bolster N: choose a creature you control with the least toughness among creatures you control
and put N +1/+1 counters on it. [CR#701.39a] -/
action bolster (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "bolster", frames := [Amount] }] },
  spelling := "bolster <amount>",
  body :=
    sequentially
      [choose
          (a
            (and
              [isCreature, actorControls,
                superlative .min (.stat .toughness)
                  (and
                    [isCreature, actorControls])])),
        putCounters amount (printedKind p1p1Counter)
          (that creature)]
}
