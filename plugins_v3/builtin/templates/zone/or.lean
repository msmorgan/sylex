/-- "<loc> or <loc>", as "on the top or bottom of your library". -/
template or (locs : List Location) : Location := {
  body := coord .or locs
}
