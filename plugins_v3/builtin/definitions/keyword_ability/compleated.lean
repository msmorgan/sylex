/-- "Compleated": "If this permanent would enter with one or more loyalty counters on it and the
player who cast it chose to pay life for any part of its cost represented by Phyrexian mana
symbols, it instead enters the battlefield with that many loyalty counters minus two for each of
those mana symbols." [CR#702.150a] Two fewer for each is the entry's loyalty counters lowered by
twice their number. -/
keyword compleated : Ability := {
  lexical := { lemma := "compleated" },
  spelling := "compleated",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.replace
          (Semantics.Templates.WouldClause.would (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisPermanent)
            (Semantics.Templates.Action.enter Option.none
              [Semantics.Templates.MoveRider.withCounters Semantics.Templates.Amount.anyAmount
                  (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.loyaltyCounter) Semantics.EntryCounterMark.fresh]))
          (Semantics.Templates.Replacement.doIf
            (Semantics.Templates.Condition.compareAmt (Semantics.Templates.Amount.phyrexianPaidWithLife Semantics.Templates.NounPhrase.thisSpell)
              Semantics.Comparator.atLeast 1)
            (Semantics.Templates.Replacement.withRiders
              [Semantics.Templates.MoveRider.withCounters
                  (Semantics.Templates.Amount.times 2 (Semantics.Templates.Amount.phyrexianPaidWithLife Semantics.Templates.NounPhrase.thisSpell))
                  (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.loyaltyCounter)
                  Semantics.EntryCounterMark.fewer])))]
}
