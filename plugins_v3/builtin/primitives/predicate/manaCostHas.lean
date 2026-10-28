/-- Authoring primitive for `Predicate.manaCostHas`. -/
primitive manaCostHas (symbol : ManaSymbol) : Predicate := {
  spelling := "with <symbol> in its mana cost",
  body :=
    Semantics.Primitives.Predicate.manaCostHas symbol
}
