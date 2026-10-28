/-- Blight N: put N −1/−1 counters on a creature you control. [CR#701.68a] -/
action blight (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "blight", frames := [Amount] }] },
  spelling := "blight <amount>",
  body :=
    putCounters amount m1m1Counter
        (a (and [isCreature, actorControls]))
}
