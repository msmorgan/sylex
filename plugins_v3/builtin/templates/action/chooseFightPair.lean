template chooseFightPair : Action := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := choose (both (target (and [isCreature, youControl])) (target (and [isCreature, youDontControl])))
}
