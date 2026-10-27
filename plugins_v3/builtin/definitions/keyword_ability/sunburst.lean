/-- "Sunburst": "If this object is entering as a creature, ignoring any type-changing effects
that would affect it, it enters with a +1/+1 counter on it for each color of mana spent to cast
it. Otherwise, it enters with a charge counter on it for each color of mana spent to cast it."
[CR#702.44a] It adds counters only as the object enters from the stack as a resolving spell on
which colored mana was spent [CR#702.44b]. -/
keyword sunburst : Ability := {
  lexical := { lemma := "sunburst" },
  spelling := "sunburst",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.replace
          (Semantics.Templates.WouldClause.would (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisPermanent)
            (Semantics.Templates.Action.enter (Option.some Semantics.Templates.Location.stack)))
          (Semantics.Templates.Replacement.doIf
            (Semantics.Templates.Condition.and
              [Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisPermanent
                  (Semantics.Templates.Predicate.hasTypeIgnoringTypeChanges Semantics.CardType.creature),
                Semantics.Templates.Condition.coloredManaSpentToCast Semantics.Templates.NounPhrase.thisSpell])
            (Semantics.Templates.Replacement.withRiders
              [Semantics.Templates.MoveRider.withCounters (Semantics.Templates.Amount.colorsSpentToCast Semantics.Templates.NounPhrase.thisSpell)
                  (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter) Semantics.EntryCounterMark.fresh]))),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.replace
          (Semantics.Templates.WouldClause.would (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisPermanent)
            (Semantics.Templates.Action.enter (Option.some Semantics.Templates.Location.stack)))
          (Semantics.Templates.Replacement.doIf
            (Semantics.Templates.Condition.and
              [Semantics.Templates.Condition.not
                  (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisPermanent
                    (Semantics.Templates.Predicate.hasTypeIgnoringTypeChanges Semantics.CardType.creature)),
                Semantics.Templates.Condition.coloredManaSpentToCast Semantics.Templates.NounPhrase.thisSpell])
            (Semantics.Templates.Replacement.withRiders
              [Semantics.Templates.MoveRider.withCounters (Semantics.Templates.Amount.colorsSpentToCast Semantics.Templates.NounPhrase.thisSpell)
                  (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.chargeCounter)
                  Semantics.EntryCounterMark.fresh])))]
}
