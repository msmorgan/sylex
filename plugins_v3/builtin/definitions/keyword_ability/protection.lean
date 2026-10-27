/-- "Protection from [quality]". Targeting, enchanting and damage are a "permanent or player"'s
[CR#702.16b,702.16c,702.16e], so those clauses are written over the holder (`this_`); equipping
and fortifying are a permanent's [CR#702.16d] and blocking a creature's [CR#702.16f]. -/
keyword protection (quality : Predicate) : Ability := {
  lexical := { lemma := "protection", parameters := [[Quality (Some ("from"))]] },
  -- TODO(spelling): Protection from red / protection from creatures; the quality needs
  -- context-dependent article or number inflection, not literal substitution.
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.cant
          (Semantics.Templates.Subject.by_
            (Semantics.Templates.NounPhrase.allOf
              (Semantics.Templates.Predicate.or
                [Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isSpell, quality],
                  Semantics.Templates.Predicate.abilityOf
                    (Semantics.Templates.NounPhrase.allOf
                      (Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isSource, quality]))])))
          (Semantics.Templates.Action.target Semantics.Templates.NounPhrase.this_)),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.cant Semantics.Templates.Subject.any
          (Semantics.Definitions.attachTo
            (Semantics.Templates.NounPhrase.allOf
              (Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.aura, quality]))
            Semantics.Templates.NounPhrase.this_)),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.cant Semantics.Templates.Subject.any
          (Semantics.Definitions.attachTo
            (Semantics.Templates.NounPhrase.allOf
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.or
                    [Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.equipment,
                      Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.fortification],
                  quality]))
            Semantics.Templates.NounPhrase.thisPermanent)),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.replace
          (Semantics.Templates.WouldClause.would
            (Semantics.Templates.Subject.by_
              (Semantics.Templates.NounPhrase.allOf (Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isSource, quality])))
            (Semantics.Templates.Action.dealDamage Semantics.Templates.Amount.anyAmount Semantics.Templates.NounPhrase.this_))
          (Semantics.Templates.Replacement.instead (Semantics.Templates.Action.preventDamage Semantics.Templates.PreventCut.allDamage))),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.cant
          (Semantics.Templates.Subject.by_
            (Semantics.Templates.NounPhrase.allOf (Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isCreature, quality])))
          (Semantics.Templates.Action.block Semantics.Templates.NounPhrase.thisCreature))]
}
