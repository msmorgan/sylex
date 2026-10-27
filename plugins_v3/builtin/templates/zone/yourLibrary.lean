/-- "your library" -/
template yourLibrary : Location := {
  spelling := "your library",
  body :=
    Semantics.Templates.Location.libraryOf Semantics.Templates.NounPhrase.you
}
