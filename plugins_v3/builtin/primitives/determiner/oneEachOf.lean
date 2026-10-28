/-- Choose one member of the pool for each listed role. -/
primitive oneEachOf (pool : NounPhrase) (roles : List Predicate := []) : NounPhrase := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.NounPhrase.oneEachOf roles pool
}
