primitive hasPossessor (axis : PossessorAxis) (possessor : NounPhrase) : Predicate := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Predicate.hasPossessor axis possessor
}
