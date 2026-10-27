/-- "<performer> explores" (`Definition`, [CR#701.44a]). -/
action explore : Action := {
  lexical := { verbs := [{ lemma := "explore", frames := [Intransitive, Monotransitive] }] },
  permanent := performs,
  spelling := "explore",
  body :=
    Semantics.Templates.Action.instruct (Semantics.Templates.NounPhrase.controllerOf Semantics.Templates.NounPhrase.theActor)
      (Semantics.Templates.Action.sequentially
        [Semantics.Definitions.reveal
            (Semantics.Templates.NounPhrase.slice Semantics.PileEnd.top 1 (Semantics.Templates.Location.libraryOf Semantics.Templates.NounPhrase.actor)),
          Semantics.Templates.Action.doIf
            (Semantics.Templates.Condition.is (Semantics.Templates.NounPhrase.that Semantics.NounWord.card) Semantics.Templates.Predicate.isLand)
            (Semantics.Templates.Action.putInto (Semantics.Templates.NounPhrase.that Semantics.NounWord.card)
              (Semantics.Templates.Location.zone Semantics.Zone.hand
                (Option.some (Semantics.Templates.NounPhrase.ownerOf (Semantics.Templates.NounPhrase.that Semantics.NounWord.card)))))
            (Option.some
              (Semantics.Templates.Action.sequentially
                [Semantics.Templates.Action.putCounters 1 (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
                    Semantics.Templates.NounPhrase.theActor,
                  Semantics.Templates.Action.may
                    (Semantics.Templates.Action.putInto (Semantics.Templates.NounPhrase.that Semantics.NounWord.card)
                      Semantics.Templates.Location.graveyard)]))])
}
