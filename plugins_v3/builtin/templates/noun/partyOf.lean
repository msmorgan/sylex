/-- "[a player]'s party" [CR#700.8]. -/
template partyOf (player : NounPhrase) : NounPhrase := {
  -- TODO(spelling): <player>'s party; string substitution cannot select possessive or subject
  -- agreement.
  body :=
    .oneEachOf partyRoles (allOf (.and [isCreature, .hasPossessor .controller player]))
}
