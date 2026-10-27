keyword cipher : Ability := {
  lexical := { lemma := "cipher" },
  spelling := "cipher",
  body :=
    [Semantics.Templates.Ability.spell
        (Semantics.Templates.Action.doIf (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.this_ Semantics.Templates.Predicate.isCard)
          (Semantics.Templates.Action.may
            (Semantics.Templates.Action.sequentially
              [Semantics.Definitions.exile Semantics.Templates.NounPhrase.thisSpell,
                Semantics.Templates.Action.relate Semantics.Templates.NounPhrase.this_ Semantics.Definitions.encodedOn
                  (Semantics.Templates.NounPhrase.a
                    (Semantics.Templates.Predicate.and
                      [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.youControl]))]))),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.abilityGrant
          (Semantics.Templates.NounPhrase.counterpart Semantics.Definitions.encodedOn Semantics.Templates.NounPhrase.this_)
          (Semantics.Templates.Ability.whenever
            (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisCreature)
              (Semantics.Templates.Action.dealDamage Semantics.Templates.Amount.anyAmount (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isPlayer)
                (Option.some Semantics.DamageKind.combat)))
            (Semantics.Templates.Action.may
              (Semantics.Templates.Action.sequentially
                [Semantics.Templates.Action.copy Semantics.CopySort.fromCardZone
                    (Semantics.Templates.NounPhrase.the
                      (Semantics.Templates.Predicate.and
                        [Semantics.Templates.Predicate.isCard,
                          Semantics.Templates.Predicate.related Semantics.Definitions.encodedOn
                            (Option.some Semantics.Templates.NounPhrase.thisCreature)]))
                    1 [],
                  Semantics.Templates.Action.may
                    (Semantics.Definitions.cast (Semantics.Templates.NounPhrase.that Semantics.NounWord.copy) Option.none
                      Semantics.Templates.PlayPayment.withoutPaying)]))))]
}
