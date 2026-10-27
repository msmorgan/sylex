/-- "<subject> deals [<amount>] [<kind>] damage [to <to>]" -/
primitive dealDamage (amount : Amount := anyAmount)
    (to : NounPhrase := Templates.NounPhrase.any) (kind : Option DamageKind := none) : Action := {
  -- TODO(spelling): Context-dependent wording: <subject> deals [<amount>] [<kind>] damage [to <to>]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Action.dealDamage amount to kind
}
