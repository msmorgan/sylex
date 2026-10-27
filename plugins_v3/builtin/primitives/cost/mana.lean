primitive mana (cost : List ManaSymbol) : Cost := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Cost.mana cost
}
