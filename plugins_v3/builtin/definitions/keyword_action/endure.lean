/-- Endure N: the permanent's controller creates an N/N white Spirit creature token unless they put
N +1/+1 counters on that permanent. [CR#701.63a] -/
action endure (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "endure", frames := [Amount] }] },
  permanent := performs,
  spelling := "endure <amount>",
  body :=
    instruct (controllerOf theActor)
        (chooseOne
          (putCounters amount p1p1Counter theActor)
          (create 1 (creatureTokenOf amount amount [white] [spirit])))
}
