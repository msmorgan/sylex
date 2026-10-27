/-- "the bottom card of your library" -/
template bottomCard : NounPhrase := {
  spelling := "the bottom card of your library",
  body :=
    Semantics.Templates.NounPhrase.slice Semantics.PileEnd.bottom 1 (Semantics.Templates.Location.libraryOf Semantics.Templates.NounPhrase.you)
}
