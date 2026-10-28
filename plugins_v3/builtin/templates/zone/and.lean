/-- "<loc> and <loc>", as "graveyard, hand, and library". -/
template and (locs : List Location) : Location := {
  body := coord .and locs
}
