/-- "<player>'s graveyard" -/
primitive graveyardOf (player : NounPhrase) : Location := {
  spelling := "<player>'s graveyard",
  body :=
    Semantics.Primitives.Location.zone Semantics.Zone.graveyard (Option.some player)
}
