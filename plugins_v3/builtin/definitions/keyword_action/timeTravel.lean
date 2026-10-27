/-- "time travel" [CR#701.56a]: any number of permanents the performer controls with time counters
and/or suspended cards they own in exile with time counters, a suspended card being one in exile
that has suspend and a time counter [CR#702.62b]; for each, put or remove a time counter
(`Definition`). -/
action timeTravel : Action := {
  lexical := { verbs := [{ lemma := "time travel", frames := [Intransitive] }] },
  spelling := "time travel",
  body :=
    Semantics.Templates.Action.sequentially
      [Semantics.Templates.Action.choose
          (Semantics.Templates.NounPhrase.counted Semantics.Templates.Quantity.anyNumber
            (Semantics.Templates.Predicate.andOr
              [Semantics.Templates.Predicate.and
                  [Semantics.Templates.Predicate.isPermanent, Semantics.Templates.Predicate.actorControls,
                    Semantics.Templates.Predicate.hasCounter (Option.some Semantics.Definitions.timeCounter)],
                Semantics.Templates.Predicate.and
                  [Semantics.Templates.Predicate.isCard, Semantics.Templates.Predicate.in_ Semantics.Templates.Location.exileZone,
                    Semantics.Templates.Predicate.ownedBy Semantics.Templates.NounPhrase.actor,
                    Semantics.Templates.Predicate.hasKeyword (Semantics.Templates.KeywordTerm.theKeyword Semantics.Definitions.suspend.label),
                    Semantics.Templates.Predicate.hasCounter (Option.some Semantics.Definitions.timeCounter)]])),
        Semantics.Templates.Action.doForEach Semantics.Templates.NounPhrase.them
          (Semantics.Templates.Action.chooseOne
            (Semantics.Templates.Action.putCounters 1 (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.timeCounter)
              Semantics.Templates.NounPhrase.it)
            (Semantics.Templates.Action.removeCounters (Semantics.Templates.Quantity.oneThrough 1)
              (Option.some (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.timeCounter)) Semantics.Templates.NounPhrase.it))]
}
