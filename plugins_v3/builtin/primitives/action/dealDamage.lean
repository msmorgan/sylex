/-- "<subject> deals [<amount>] [<kind>] damage [to <to>]" -/
primitive dealDamage (amount : Amount := anyAmount)
    (to : NounPhrase := any) (kind : Option DamageKind := none) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by kind.
  body :=
    Semantics.Primitives.Action.dealDamage amount to kind
}
