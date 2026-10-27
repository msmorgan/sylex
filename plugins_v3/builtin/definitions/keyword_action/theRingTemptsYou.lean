/-- "The Ring tempts you": the Ring emblem if you have none, then you choose a creature you
control as your Ring-bearer (`Definition`, [CR#701.54a,701.54c]). The emblem's abilities count
the times the Ring has tempted you by the deed's own steps, `tempting`. -/
action theRingTemptsYou : Action := {
  lexical := { verbs := [{ lemma := "tempt", frames := [Monotransitive] }], nouns := ["Ring"] },
  spelling := "The Ring tempts you",
  body :=
    have ringBearers :=
      Semantics.Templates.Predicate.and
        [Semantics.Templates.Predicate.hasDesignation Semantics.Definitions.ringBearer, Semantics.Templates.Predicate.youControl];
    have tempting :=
      [Semantics.Templates.Action.choose
          (Semantics.Templates.NounPhrase.a
            (Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.actorControls])),
        Semantics.Templates.Action.gainDesignation (Semantics.Templates.NounPhrase.that Semantics.Templates.NounWord.creature)
          Semantics.Definitions.ringBearer];
    have timesTempted :=
      Semantics.Templates.Amount.eventCount
        (Semantics.Action.ownScope (Option.some (Semantics.Deed.action "theRingTemptsYou"))
          (Semantics.Templates.Action.sequentially tempting))
        (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you) Semantics.Templates.Duration.thisGame;
    Semantics.Templates.Action.sequentially
      (Semantics.Templates.Action.doIf
          (Semantics.Templates.Condition.not
            (Semantics.Templates.Condition.exists_
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.isEmblem, Semantics.Templates.Predicate.named (Semantics.Templates.NameSource.printedName "The Ring"),
                  Semantics.Templates.Predicate.ownedBy Semantics.Templates.NounPhrase.theActor])))
          (Semantics.Templates.Action.getNamedEmblem "The Ring"
            [Semantics.Templates.Ability.static
                (Semantics.Templates.StaticSpec.characteristicChange (Semantics.Templates.NounPhrase.allOf ringBearers)
                  [Semantics.Templates.CharacteristicEdit.characteristicEditTypeLine Semantics.QualityOp.adds
                      { supertypes := Option.some [Semantics.Supertype.legendary] }]),
              Semantics.Templates.Ability.static
                (Semantics.Templates.StaticSpec.cant
                  (Semantics.Templates.Subject.by_
                    (Semantics.Templates.NounPhrase.allOf
                      (Semantics.Templates.Predicate.and
                        [Semantics.Templates.Predicate.isCreature,
                          Semantics.Templates.Predicate.hasPower Semantics.Comparator.greater
                            (Semantics.Templates.Amount.powerOf (Semantics.Templates.NounPhrase.a ringBearers))])))
                  (Semantics.Templates.Action.block (Semantics.Templates.NounPhrase.allOf ringBearers))),
              Semantics.Templates.Ability.static
                (Semantics.Templates.StaticSpec.onlyWhile
                  (Semantics.Templates.StaticSpec.abilityGrant Semantics.Templates.NounPhrase.this_
                    (Semantics.Templates.Ability.whenever
                      (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor (Semantics.Templates.NounPhrase.a ringBearers))
                        Semantics.Templates.Action.attack)
                      (Semantics.Templates.Action.sequentially
                        [Semantics.Templates.Action.draw 1,
                          Semantics.Definitions.discard
                            (Semantics.Templates.NounPhrase.a (Semantics.Templates.Predicate.cardIn Semantics.Templates.Location.hand))])))
                  (Semantics.Templates.Condition.compareAmt timesTempted Semantics.Comparator.atLeast 2)),
              Semantics.Templates.Ability.static
                (Semantics.Templates.StaticSpec.onlyWhile
                  (Semantics.Templates.StaticSpec.abilityGrant Semantics.Templates.NounPhrase.this_
                    (Semantics.Templates.Ability.whenever
                      (Semantics.Templates.GameEvent.does
                        (Semantics.Templates.Subject.by_ (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isCreature))
                        (Semantics.Templates.Action.block (Semantics.Templates.NounPhrase.a ringBearers)))
                      (Semantics.Templates.Action.atNext Semantics.TurnPart.endOfCombat
                        (Semantics.Templates.Action.controllerSacrifices
                          (Semantics.Templates.NounPhrase.that Semantics.Templates.NounWord.creature)))))
                  (Semantics.Templates.Condition.compareAmt timesTempted Semantics.Comparator.atLeast 3)),
              Semantics.Templates.Ability.static
                (Semantics.Templates.StaticSpec.onlyWhile
                  (Semantics.Templates.StaticSpec.abilityGrant Semantics.Templates.NounPhrase.this_
                    (Semantics.Templates.Ability.whenever
                      (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor (Semantics.Templates.NounPhrase.a ringBearers))
                        (Semantics.Templates.Action.dealDamage Semantics.Templates.Amount.anyAmount
                          (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isPlayer) (Option.some Semantics.DamageKind.combat)))
                      (Semantics.Templates.Action.instruct Semantics.Templates.NounPhrase.eachOpponent (Semantics.Templates.Action.loseLife 3))))
                  (Semantics.Templates.Condition.compareAmt timesTempted Semantics.Comparator.atLeast 4))]) ::
        tempting)
}
