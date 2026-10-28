/-- Clash: reveal the top card of your library, then you may put it on the bottom of your library.
[CR#701.30a] -/
action clash : Action := {
  lexical := { verbs := [{ lemma := "clash", frames := [(Prepositional ["with"]), Intransitive] }] },
  spelling := "clash",
  body :=
    sequentially
      [expose reveal
          (cards
            (slice top 1
              (libraryOf actor))),
        may
          (putInto (that card) onBottom)]
}
