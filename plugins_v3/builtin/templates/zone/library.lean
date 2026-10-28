/-- "its owner's library": a library scoped to nobody, which an object goes to as its owner's
[CR#400.3]. -/
-- [CR#400.3]
template library : Location := {
  spelling := "its owner's library",
  body := zone .library none
}
