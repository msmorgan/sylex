template zoyowasJusticeBody : Action := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := sequentially
    [ instruct (ownerOf (target (and [or [isArtifact, isCreature], hasManaValue .atLeast 1])))
        (shuffleInto it),
      instruct (that player) (discover (statOf (.stat .manaValue) it)) ]
}
