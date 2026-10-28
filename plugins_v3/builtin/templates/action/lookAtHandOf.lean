/-- "look at <player>'s hand" -/
template lookAtHandOf (player : NounPhrase) : Action := {
  spelling := "look at <player>'s hand",
  body :=
    expose Semantics.ExposeVerb.lookAt (exposedZone (handOf player))
}
