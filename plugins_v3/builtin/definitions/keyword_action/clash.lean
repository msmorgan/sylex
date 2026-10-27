action clash : Action := {
  lexical := { verbs := [{ lemma := "clash", frames := [(Prepositional ["with"]), Intransitive] }] },
  spelling := "clash",
  body :=
    sequentially
        [ .expose .reveal (.cards (.slice .top 1 (libraryOf .actor))),
          may (putInto (that .card) onBottom) ]
}
