/-- "your graveyard" -/
template yourGraveyard : Location := {
  spelling := "your graveyard",
  body :=
    Semantics.Templates.Location.graveyardOf Semantics.Templates.NounPhrase.you
}
