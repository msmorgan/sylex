/-- "search your library and/or graveyard for <for_>", "your" written once. -/
template searchLibraryOrGraveyard (for_ : NounPhrase) : Action := {
  spelling := "search your library and/or graveyard for <for_>",
  body :=
    search (andOr [yourLibrary, yourGraveyard]) for_
}
