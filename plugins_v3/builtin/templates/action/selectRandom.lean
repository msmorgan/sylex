template selectRandom (quantity : Quantity) (what : Predicate) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Templates.Action.choose (Semantics.Templates.NounPhrase.countedAtRandom quantity what)
}
