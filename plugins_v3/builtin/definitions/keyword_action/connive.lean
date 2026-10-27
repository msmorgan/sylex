/-- "<performer> connives [N]" (`Definition`, [CR#701.50a,701.50d]): loops over
`"each nonland card discarded this way"`
(`allOf(and([not(land), card, did(any, discard(which), thisWay)]))`). -/
action connive (amount : Amount := 1) : Action := {
  lexical := { verbs := [{ lemma := "connive", frames := [Intransitive, Amount] }] },
  permanent := performs,
  spelling := "connive <amount>",
  body :=
    Semantics.Templates.Action.instruct (Semantics.Templates.NounPhrase.controllerOf Semantics.Templates.NounPhrase.theActor)
      (Semantics.Templates.Action.sequentially
        [Semantics.Templates.Action.draw amount,
          Semantics.Definitions.discard
            (Semantics.Templates.NounPhrase.counted (Semantics.Templates.Quantity.exactlyOf amount) Semantics.Templates.Predicate.isCard),
          Semantics.Templates.Action.doForEach
            (Semantics.Templates.NounPhrase.allOf
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.non Semantics.Templates.Predicate.isLand, Semantics.Templates.Predicate.isCard,
                  Semantics.Templates.Predicate.predicateDid
                    { subj := Semantics.Templates.Subject.any, action := Semantics.Definitions.discard Semantics.Templates.NounPhrase.which,
                      within := Option.some Semantics.Templates.LookbackSpan.thisWay }]))
            (Semantics.Templates.Action.putCounters 1 (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
              Semantics.Templates.NounPhrase.theActor)])
}
