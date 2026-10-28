/-- "<loc> and/or <loc>", as "library and/or graveyard". -/
template andOr (locs : List Location) : Location := {
  body := coord .andOr locs
}
