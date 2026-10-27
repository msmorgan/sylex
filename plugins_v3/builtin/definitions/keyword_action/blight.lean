action blight (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "blight", frames := [Amount] }] },
  spelling := "blight <amount>",
  body :=
    putCounters amount m1m1Counter
        (a (and [isCreature, actorControls]))
}
