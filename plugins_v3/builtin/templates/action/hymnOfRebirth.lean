template hymnOfRebirth : Action := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := putOntoBattlefieldUnderYourControl (target (and [isCreature, in_ graveyard]))
    (from_ := graveyard)
}
