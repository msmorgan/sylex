template selectRandom (quantity : Quantity) (what : Predicate) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    choose (countedAtRandom quantity what)
}
