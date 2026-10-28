/-- Authoring primitive for `Predicate.abilityHead`. -/
primitive abilityHead (class_ : AbilityClass) : Predicate := {
  spelling := "<class_> ability",
  body :=
    Semantics.Primitives.Predicate.abilityHead class_
}
