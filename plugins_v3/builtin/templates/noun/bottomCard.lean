/-- "the bottom card of your library" -/
template bottomCard : NounPhrase := {
  spelling := "the bottom card of your library",
  body :=
    slice Semantics.PileEnd.bottom 1 (libraryOf you)
}
