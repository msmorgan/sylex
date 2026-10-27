template gainLife (amount : Amount) : Action := {
  spelling := "gain <amount> life",
  body :=
    Semantics.Templates.Action.changeLife (Semantics.Templates.up amount)
}
