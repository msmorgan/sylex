/-- A cost given as a list of mana symbols. -/
primitive mana (cost : List ManaSymbol) : Cost := {
  body :=
    Semantics.Primitives.Cost.mana cost
}
