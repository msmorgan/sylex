/-- Authoring primitive for `Predicate.hasPossessor`. -/
primitive hasPossessor (axis : PossessorAxis) (possessor : NounPhrase) : Predicate := {
  body :=
    Semantics.Primitives.Predicate.hasPossessor axis possessor
}
