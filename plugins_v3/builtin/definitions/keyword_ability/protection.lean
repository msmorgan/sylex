/-- "Protection from [quality]". Targeting, enchanting and damage are a "permanent or player"'s
[CR#702.16b,702.16c,702.16e], so those clauses are written over the holder (`this_`); equipping
and fortifying are a permanent's [CR#702.16d] and blocking a creature's [CR#702.16f]. -/
keyword protection (quality : Predicate) : Ability := {
  lexical := { lemma := "protection", parameters := [[Quality (Some ("from"))]] },
  -- TODO(spelling): Protection from red / protection from creatures; the quality needs
  -- context-dependent article or number inflection, not literal substitution.
  body :=
    [static
        (cant
          (by_
            (allOf
              (or
                [and [isSpell, quality],
                  abilityOf
                    (allOf
                      (and [isSource, quality]))])))
          (target this_)),
      static
        (cant Templates.Subject.any
          (attachTo
            (allOf
              (and [hasSubtype aura, quality]))
            this_)),
      static
        (cant Templates.Subject.any
          (attachTo
            (allOf
              (and
                [or
                    [hasSubtype equipment,
                      hasSubtype fortification],
                  quality]))
            thisPermanent)),
      static
        (replace
          (would
            (by_
              (allOf (and [isSource, quality])))
            (dealDamage anyAmount this_))
          (instead (preventDamage allDamage))),
      static
        (cant
          (by_
            (allOf (and [isCreature, quality])))
          (block thisCreature))]
}
