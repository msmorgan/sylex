template creaturePutOntoTheBattlefieldWith (source : NounPhrase) : Predicate := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := and [isCreature, did (by_ source) (putOntoBattlefield which)]
}
