/-- Authoring primitive for `Predicate.colorCount`. -/
primitive colorCount (comparator : Comparator) (count : Nat) : Predicate := {
  body :=
    Semantics.Primitives.Predicate.colorCount comparator count
}
