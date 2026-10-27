/-- "Freerunning [cost]": "You may pay [cost] rather than pay this spell's mana cost if a player
was dealt combat damage this turn by a creature that, at the time it dealt that damage, was an
Assassin creature or a commander under your control." [CR#702.173a] The lookback's description
reads the creature as it dealt the damage, as prowl's does. -/
keyword freerunning (cost : Cost) : Ability := {
  lexical := { lemma := "freerunning", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): freerunning [mana cost] / freerunning—[verbal cost]; separator and terminal
  -- period depend on the Cost structure.
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.onlyWhile (Semantics.Templates.StaticSpec.altCost Semantics.Templates.NounPhrase.this_ (Option.some cost))
          (Semantics.Templates.Condition.did
            (Semantics.Templates.Subject.actor
              (Semantics.Templates.NounPhrase.a
                (Semantics.Templates.Predicate.and
                  [Semantics.Templates.Predicate.isCreature,
                    Semantics.Templates.Predicate.or
                      [Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.assassin,
                        Semantics.Templates.Predicate.hasDesignation Semantics.Definitions.commander],
                    Semantics.Templates.Predicate.youControl])))
            (Semantics.Templates.Action.dealDamage Semantics.Templates.Amount.anyAmount (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isPlayer)
              (Option.some Semantics.DamageKind.combat))
            (Semantics.Templates.LookbackSpan.within Semantics.Templates.Duration.soFarThisTurn)))]
}
