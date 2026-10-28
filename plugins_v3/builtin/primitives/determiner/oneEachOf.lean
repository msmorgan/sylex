/-- Choose one member of the pool for each listed role. -/
primitive oneEachOf (pool : NounPhrase) (roles : List Predicate := []) : NounPhrase := {
  body :=
    Semantics.Primitives.NounPhrase.oneEachOf roles pool
}
