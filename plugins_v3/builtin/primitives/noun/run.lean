/-- The cards from one end of a pile through the stopping predicate. -/
primitive run (end_ : PileEnd) (stop : Predicate) (pile : Location) : NounPhrase := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.NounPhrase.run end_ stop pile
}
