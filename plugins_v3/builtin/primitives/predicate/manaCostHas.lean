/-- Authoring primitive for `Predicate.manaCostHas`. -/
primitive manaCostHas (symbol : ManaSymbol) : Predicate := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Predicate.manaCostHas symbol
}
