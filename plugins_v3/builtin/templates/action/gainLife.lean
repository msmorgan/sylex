template gainLife (amount : Amount) : Action := {
  spelling := "gain <amount> life",
  body :=
    changeLife (up amount)
}
