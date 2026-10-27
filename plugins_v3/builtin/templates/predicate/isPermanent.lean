/-- "permanent": an object on the battlefield [CR#110.1]. -/
-- [CR#110.1]
template isPermanent : Predicate := {
  spelling := "permanent",
  body :=
    Semantics.Templates.Predicate.in_ Semantics.Templates.Location.battlefield
}
