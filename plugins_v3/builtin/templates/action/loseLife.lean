template loseLife (amount : Amount) : Action := {
  spelling := "lose <amount> life",
  body :=
    changeLife (down amount)
}
