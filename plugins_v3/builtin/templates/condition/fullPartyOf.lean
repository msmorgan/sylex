/-- "<player> has a full party": four creatures in that party [CR#700.8c]. -/
template fullPartyOf (player : NounPhrase) : Condition := {
  -- TODO(spelling): <player> has/have a full party; string substitution cannot select possessive or
  -- subject agreement.
  body :=
    .compareAmt (partySizeOf player) .eq 4
}
