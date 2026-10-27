/-- "the command zone", scoped to nobody. -/
primitive commandZone : Location := {
  spelling := "the command zone",
  body :=
    Semantics.Primitives.Location.zone Semantics.Zone.command Option.none
}
