/-- Psychic Purge -/
card := {
  name := "Psychic Purge",
  cost := [blue],
  types := [sorcery],
  text := [
    spell (instruct this_ (dealDamage 1 (target isAnyTarget))),
    when
      (causes (bySource (a [or [isSpell, abilityHead anyOnStack], anOpponentControls]))
         (does you (discard this_)))
      (instruct (that player) (loseLife 5))
  ]
}
