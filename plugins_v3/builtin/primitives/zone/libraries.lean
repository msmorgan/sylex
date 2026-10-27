/-- "libraries": every player's library, as "Players can't search libraries". -/
primitive libraries : Location := {
  spelling := "libraries",
  body :=
    Semantics.Primitives.Location.zone Semantics.Zone.library Option.none
}
