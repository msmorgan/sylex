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
