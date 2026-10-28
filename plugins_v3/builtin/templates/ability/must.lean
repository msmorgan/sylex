/-- "<subj> <action>s if able", "<subj> must <action>". A requirement with the unspecified agent
`any` is one requirement on the whole clause: "this creature must be blocked if able" is one
requirement on the declaration of blockers [CR#509.1c], met when at least one creature blocks it.
`by_ (allOf …)` instead puts a requirement on each creature ("must be blocked by all creatures
able to block it"). -/
-- [CR#509.1c]
template must (subj : Subject) (action : Action)
    (timing : Option Timing := none)
    (asThough : Option AsThough := none)
    (except : Option DeonticExcept := none) : StaticSpec := {
  -- TODO(spelling): <subj> <action>s if able; string substitution cannot inflect verbs, nouns, or ordinals.
  body :=
    staticSpecMust subj action { timing := timing, asThough := asThough, except := except }
}
