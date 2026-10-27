keyword bloodthirst (amount : Amount) : Ability := {
  lexical := { lemma := "bloodthirst", parameters := [[Amount]] },
  spelling := "bloodthirst <amount>",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.replace
          (Semantics.Templates.WouldClause.would (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisPermanent)
            Semantics.Templates.Action.enter)
          (Semantics.Templates.Replacement.doIf
            (Semantics.Templates.Condition.was
              (Semantics.Templates.Action.dealDamage Semantics.Templates.Amount.anyAmount Semantics.Templates.NounPhrase.anOpponent)
              (Semantics.Templates.LookbackSpan.within Semantics.Templates.Duration.soFarThisTurn))
            (Semantics.Templates.Replacement.withRiders
              [Semantics.Templates.MoveRider.withCounters amount (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
                  Semantics.EntryCounterMark.fresh])))]
}
