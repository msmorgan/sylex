/-- [CR#120.3g,702.164c] "Combat damage dealt to a player by a creature with toxic causes that
creature's controller to give the player a number of poison counters equal to that creature's
total toxic value, in addition to the damage's other results." One row for the creature,
reading the sum of its toxic abilities' N values [CR#702.164b], not one per toxic ability: the
keyword is bodiless, so combat damage from a creature with toxic 2 and toxic 1 gives three poison
counters once. -/
conferral toxicPoison := {

  body :=
    {
      scope :=
        Semantics.Templates.Predicate.and
          [Semantics.Templates.Predicate.isCreature,
            Semantics.Templates.Predicate.hasKeyword (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "toxic"))],
      confer :=
        Semantics.Conferral.property
          (Semantics.Templates.StaticSpec.asEvent
            (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.this_)
              (Semantics.Templates.Action.dealDamage Semantics.Templates.Amount.anyAmount
                (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isPlayer) (Option.some Semantics.DamageKind.combat)))
            (Semantics.Templates.Action.putCounters (Semantics.Templates.Amount.totalToxicValue Semantics.Templates.NounPhrase.this_)
              (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.poison)
              (Semantics.Templates.NounPhrase.that Semantics.Templates.NounWord.player))) }
}
