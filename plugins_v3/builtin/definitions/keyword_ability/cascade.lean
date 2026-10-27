/-- "Cascade" with its reminder text [CR#702.85a]. -/
keyword cascade : Ability := {
  lexical := { lemma := "cascade", verbs := [{ lemma := "cascade", frames := [Intransitive] }] },
  spelling := "cascade",
  body :=
    have cheaper :=
      Semantics.Templates.Predicate.predicateCompare [Semantics.ProjAxis.stat Semantics.Stat.manaValue] Semantics.Comparator.less
        (Semantics.Templates.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.manaValue) Semantics.Templates.NounPhrase.thisSpell);
    [Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
          (Semantics.Definitions.cast Semantics.Templates.NounPhrase.thisSpell))
        (Semantics.Templates.Action.sequentially
          [Semantics.Definitions.exile
              (Semantics.Templates.NounPhrase.run Semantics.PileEnd.top
                (Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.non Semantics.Templates.Predicate.isLand, cheaper])
                (Semantics.Templates.Location.libraryOf Semantics.Templates.NounPhrase.you)),
            Semantics.Templates.Action.may
              (Semantics.Definitions.castWithoutPaying (Semantics.Templates.NounPhrase.that Semantics.NounWord.card)
                (Option.some cheaper)),
            Semantics.Templates.Action.actionMove
              (Semantics.Templates.NounPhrase.allOf
                (Semantics.Templates.Predicate.and
                  [Semantics.Templates.Predicate.isCard,
                    Semantics.Templates.Predicate.was (Semantics.Definitions.exile Semantics.Templates.NounPhrase.which)
                      (Option.some Semantics.Templates.LookbackSpan.thisWay),
                    Semantics.Templates.Predicate.not
                      (Semantics.Templates.Predicate.was (Semantics.Definitions.cast Semantics.Templates.NounPhrase.which)
                        (Option.some Semantics.Templates.LookbackSpan.thisWay))]))
              Semantics.Templates.Location.wherever (Option.some Semantics.Templates.Location.onBottom) Semantics.ObservationPoint.arrival
              [Semantics.Templates.MoveRider.inOrder Semantics.Arrangement.randomOrder]])]
}
