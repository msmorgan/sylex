/-- "colorless": an object of no color [CR#105.2c]. -/
-- [CR#105.2c]
primitive isColorless : Predicate := {
  spelling := "colorless",
  body :=
    Semantics.Primitives.Predicate.colorCount Semantics.Comparator.eq 0
}
