/-- "<subject> attacks with <attackers> [<defender>]" -/
primitive attackWith (attackers : NounPhrase) (defender : Option NounPhrase := none) :
    Action := {
  -- TODO(spelling): Context-dependent wording: <subject> attacks with <attackers> [<defender>]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Action.attackWith attackers defender
}
