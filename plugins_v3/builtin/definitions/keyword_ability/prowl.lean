/-- "Prowl <cost>": cast for <cost> if a player was dealt combat damage this turn by a source
you controlled that shared a creature type with this spell [CR#702.76a]. -/
keyword prowl (cost : Cost) : Ability := {
  lexical := { lemma := "prowl", parameters := [[Cost]] },
  spelling := "prowl <cost>",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.onlyWhile (Semantics.Templates.StaticSpec.altCost Semantics.Templates.NounPhrase.this_ (Option.some cost))
          (Semantics.Templates.Condition.did
            (Semantics.Templates.Subject.actor
              (Semantics.Templates.NounPhrase.a
                (Semantics.Templates.Predicate.and
                  [Semantics.Templates.Predicate.shares (Semantics.QualitySort.subtype Semantics.CardType.creature)
                      Semantics.Agreement.share Semantics.Templates.NounPhrase.thisSpell,
                    Semantics.Templates.Predicate.youControl])))
            (Semantics.Templates.Action.dealDamage Semantics.Templates.Amount.anyAmount (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isPlayer)
              (Option.some Semantics.DamageKind.combat))
            (Semantics.Templates.LookbackSpan.within Semantics.Templates.Duration.soFarThisTurn)))]
}
