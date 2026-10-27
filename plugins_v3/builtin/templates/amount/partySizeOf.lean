/-- "the number of creatures in <player>'s party" [CR#700.8a]. -/
template partySizeOf (player : NounPhrase) : Amount := {
  -- TODO(spelling): the number of creatures in <player>'s party; string substitution cannot select
  -- possessive or subject agreement.
  body :=
    .countOf (partyOf player)
}
