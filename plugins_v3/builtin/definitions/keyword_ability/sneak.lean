keyword sneak (cost : Cost) : Ability := {
  lexical := { lemma := "sneak", parameters := [[Cost]] },
  spelling := "sneak <cost>",
  body :=
    have returnedCreaturesDefender :=
      Semantics.Templates.NounPhrase.the
        (Semantics.Templates.Predicate.and
          [Semantics.Templates.Predicate.or
              [Semantics.Templates.Predicate.isPlayer, Semantics.Templates.Predicate.isPlaneswalker, Semantics.Templates.Predicate.isBattle],
            Semantics.Templates.Predicate.inCombat Semantics.CombatRelation.attackedBy
              (Option.some
                (Semantics.Templates.NounPhrase.the
                  (Semantics.Templates.Predicate.and
                    [Semantics.Templates.Predicate.was (Semantics.Templates.Action.returnToHand Semantics.Templates.NounPhrase.which),
                      Semantics.Templates.Predicate.isCreature])))]);
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.can (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
          (Semantics.Definitions.cast Semantics.Templates.NounPhrase.thisSpell Option.none
            (Semantics.Templates.PlayPayment.payingInstead
              (Semantics.Templates.Cost.compound
                [cost,
                  Semantics.Templates.Cost.perform
                    (Semantics.Templates.Action.returnToHand
                      (Semantics.Templates.NounPhrase.a
                        (Semantics.Templates.Predicate.and
                          [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.isUnblocked,
                            Semantics.Templates.Predicate.youControl])))])))
          (Option.some
            (Semantics.Templates.Timing.and
              [Semantics.Templates.Timing.asInstant,
                Semantics.Templates.Timing.duringPart Semantics.TurnPart.declareBlockers
                  (Option.some Semantics.Templates.NounPhrase.you)]))),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.replace
          (Semantics.Templates.WouldClause.would (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisPermanent)
            Semantics.Templates.Action.enter)
          (Semantics.Templates.Replacement.doIf
            (Semantics.Templates.Condition.keywordCostPaid Semantics.Definitions.sneak.label Semantics.Templates.NounPhrase.thisPermanent)
            -- It enters tapped and attacking what the returned creature was attacking [CR#702.190b].
            (Semantics.Templates.Replacement.withRiders
              [Semantics.Templates.MoveRider.entersAs Semantics.Status.tapped,
                Semantics.Templates.MoveRider.entersAttacking (Option.some returnedCreaturesDefender)])))]
}
