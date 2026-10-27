template creatureCardInAGraveyard : Predicate := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := and [isCreature, isCard, in_ graveyard]
}
