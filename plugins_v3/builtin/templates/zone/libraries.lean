/-- "libraries": every player's library, as "Players can't search libraries". -/
template libraries : Location := {
  spelling := "libraries",
  body := zone .library none
}
