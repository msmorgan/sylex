/-- "your hand" -/
template yourHand : Location := {
  spelling := "your hand",
  body :=
    Semantics.Templates.Location.handOf Semantics.Templates.NounPhrase.you
}
