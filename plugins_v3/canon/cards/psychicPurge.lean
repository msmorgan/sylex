/-- Psychic Purge -/
card psychicPurge : Spelled :=
  { name := "Psychic Purge", cost := [pip .blue], types := [.sorcery],
    text :=
      [ spell (instruct this_ (dealDamage 1 (target isAnyTarget))),
        when
          (causes
            (bySource (a (and [or [isSpell, abilityHead .anyOnStack],
                               anOpponentControls])))
            (does you (discard this_)))
          (instruct (that player) (loseLife 5)) ] }
