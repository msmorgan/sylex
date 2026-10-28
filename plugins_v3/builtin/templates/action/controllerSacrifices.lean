/-- "<subject>'s controller sacrifices it" (`Template` using `firstMention` / `laterMention`
without `Window` plumbing, [CR#701.21a]). -/
template controllerSacrifices (subject : NounPhrase) : Action := {

  -- TODO(spelling): <subject>'s controller sacrifices it/them. String substitution cannot form the
  -- possessor or choose the agreeing pronoun.
  body :=
    instruct (possessorOf Semantics.PossessorAxis.controller subject)
      (sacrifice subject)
}
