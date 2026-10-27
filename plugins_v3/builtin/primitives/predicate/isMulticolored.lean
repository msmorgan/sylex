/-- "multicolored": an object of two or more colors [CR#105.2b]. -/
-- [CR#105.2b]
primitive isMulticolored : Predicate := {
  spelling := "multicolored",
  body :=
    Semantics.Primitives.Predicate.colorCount Semantics.Comparator.atLeast 2
}
