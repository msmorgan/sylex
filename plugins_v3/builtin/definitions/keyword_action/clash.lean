action clash : Action := {
  lexical := { verbs := [{ lemma := "clash", frames := [(Prepositional ["with"]), Intransitive] }] },
  spelling := "clash",
  body :=
    sequentially
      [expose Semantics.ExposeVerb.reveal
          (cards
            (slice Semantics.PileEnd.top 1
              (libraryOf actor))),
        may
          (putInto (that Semantics.NounWord.card) onBottom)]
}
