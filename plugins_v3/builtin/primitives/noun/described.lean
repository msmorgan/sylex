/-- A noun phrase composed from a determiner and predicate. -/
primitive described (determiner : DetPhrase) (predicate : Predicate) : NounPhrase := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.NounPhrase.described determiner predicate
}
