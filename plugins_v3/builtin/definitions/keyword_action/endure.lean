action endure (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "endure", frames := [Amount] }] },
  permanent := performs,
  spelling := "endure <amount>",
  body :=
    instruct (controllerOf theActor)
        (chooseOne
          (putCounters amount p1p1Counter theActor)
          (create 1 (creatureTokenOf amount amount [.white] [spirit])))
}
