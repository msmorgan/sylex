action clash : Action := {
  lexical := { verbs := [{ lemma := "clash", frames := [(Prepositional ["with"]), Intransitive] }] },
  spelling := "clash",
  body :=
    Semantics.Templates.Action.sequentially
      [Semantics.Templates.Action.expose Semantics.ExposeVerb.reveal
          (Semantics.Templates.Exposed.cards
            (Semantics.Templates.NounPhrase.slice Semantics.PileEnd.top 1
              (Semantics.Templates.Location.libraryOf Semantics.Templates.NounPhrase.actor))),
        Semantics.Templates.Action.may
          (Semantics.Templates.Action.putInto (Semantics.Templates.NounPhrase.that Semantics.NounWord.card) Semantics.Templates.Location.onBottom)]
}
