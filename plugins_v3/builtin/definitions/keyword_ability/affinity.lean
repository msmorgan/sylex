keyword affinity (quality : Predicate) : Ability := {
  lexical := { lemma := "affinity", parameters := [[Quality (Some ("for"))]] },
  -- TODO(spelling): Affinity for artifacts / affinity for green creatures; the quality needs
  -- context-dependent article or number inflection, not literal substitution.
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.costShift Semantics.Templates.NounPhrase.thisSpell
          (Semantics.Templates.CostShift.costShiftGeneric
            (Semantics.Templates.down
              (Semantics.Templates.Amount.forEach 1 (Semantics.Templates.Predicate.and [quality, Semantics.Templates.Predicate.youControl])))
            Option.none))]
}
