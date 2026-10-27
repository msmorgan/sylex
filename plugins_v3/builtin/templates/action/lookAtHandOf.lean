/-- "look at <player>'s hand" -/
template lookAtHandOf (player : NounPhrase) : Action := {
  spelling := "look at <player>'s hand",
  body :=
    Semantics.Templates.Action.expose Semantics.ExposeVerb.lookAt (Semantics.Templates.Exposed.exposedZone (Semantics.Templates.Location.handOf player))
}
