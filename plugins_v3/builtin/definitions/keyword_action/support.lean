/-- "support N" (`Definition`, [CR#701.41a]). -/
action support (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "support", frames := [Amount] }] },
  spelling := "support <amount>",
  body :=
    putCounters 1 (printedKind p1p1Counter)
      (eachOf
        (described (detPhraseTarget (upToOf amount))
          (and [isCreature, isOther])))
}
