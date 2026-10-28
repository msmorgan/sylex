/-- "colorless": an object of no color [CR#105.2c]. -/
-- [CR#105.2c]
template isColorless : Predicate := {
  spelling := "colorless",
  body := colorCount eq 0
}
