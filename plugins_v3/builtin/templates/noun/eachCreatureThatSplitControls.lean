template eachCreatureThatSplitControls : NounPhrase := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := each (and [isCreature, controlledBy splitOverPlaneswalker])
}
