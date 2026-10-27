template loseLife (amount : Amount) : Action := {
  spelling := "lose <amount> life",
  body :=
    Semantics.Templates.Action.changeLife (Semantics.Templates.down amount)
}
