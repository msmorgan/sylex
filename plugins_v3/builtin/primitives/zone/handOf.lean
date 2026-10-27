/-- "<player>'s hand" -/
primitive handOf (player : NounPhrase) : Location := {
  spelling := "<player>'s hand",
  body :=
    Semantics.Primitives.Location.zone Semantics.Zone.hand (Option.some player)
}
