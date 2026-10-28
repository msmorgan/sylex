/-- "<subject> becomes attacking [<defender>]" -/
template becomeAttacking (subject : NounPhrase) (defender : Option NounPhrase) : Action := {
  -- TODO(spelling): Context-dependent wording: <subject> becomes attacking [<defender>]; string substitution cannot express its omitted or inflected parts.
  body :=
    combat subject
      (participation (attacking defender))
}
