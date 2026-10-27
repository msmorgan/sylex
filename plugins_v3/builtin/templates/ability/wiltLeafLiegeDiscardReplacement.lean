template wiltLeafLiegeDiscardReplacement : Ability := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := static (replace
    (causedBy
      (bySource (a (and [or [isSpell, abilityHead .anyOnStack], anOpponentControls])))
      (would you (discard thisCard)))
    (instead (putOntoBattlefield it)))
}
