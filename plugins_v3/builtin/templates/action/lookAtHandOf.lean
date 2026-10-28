/-- "look at <player>'s hand" -/
template lookAtHandOf (player : NounPhrase) : Action := {
  spelling := "look at <player>'s hand",
  body :=
    expose .lookAt (exposedZone (handOf player))
}
