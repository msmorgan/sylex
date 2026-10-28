keyword affinity (quality : Predicate) : Ability := {
  lexical := { lemma := "affinity", parameters := [[Quality (Some ("for"))]] },
  -- TODO(spelling): Affinity for artifacts / affinity for green creatures; the quality needs
  -- context-dependent article or number inflection, not literal substitution.
  body :=
    [static
        (costShift thisSpell
          (costShiftGeneric
            (down
              (forEach 1 (and [quality, youControl])))
            none))]
}
