/-- "Starting with <first>, vote for <ballot>" (`Definition`, [CR#701.38]). -/
action voteStartingWith (first : NounPhrase) (disclosure : Disclosure)
    (ballot : Ballot) : Action := {
  -- TODO(spelling): Starting with <first>, [secretly] vote for <ballot>; string substitution cannot
  -- branch on optional fields or supply contextual agreement and morphology.
  lexical := { verbs := [{ lemma := "vote", frames := [(Prepositional ["for"]), Intransitive] }] },
  deed := "vote",
  body :=
    .vote (some first) disclosure
        ballot none
}
