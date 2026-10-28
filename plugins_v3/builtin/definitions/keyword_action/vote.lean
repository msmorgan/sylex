/-- "vote for <ballot> [<times> times]" (`Definition`, [CR#701.38]). -/
action vote (disclosure : Disclosure := .openly)
    (ballot : Ballot := anyBallot)
    (times : Option Quantity := none) : Action := {
  -- TODO(spelling): [secretly] vote for <ballot> [<times> times]; string substitution cannot branch
  -- on optional fields or supply contextual agreement and morphology.
  lexical := { verbs := [{ lemma := "vote", frames := [(Prepositional ["for"]), Intransitive] }] },
  body :=
    actionVote none disclosure ballot times
}
