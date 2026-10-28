/-- "the command zone", scoped to nobody. -/
template commandZone : Location := {
  spelling := "the command zone",
  body := zone command none
}
