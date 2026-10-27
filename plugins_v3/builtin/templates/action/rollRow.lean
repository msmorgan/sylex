/-- One row of a results table: "<results> — <action>". -/
template rollRow (results : Quantity) (action : Action) : RollRow := {
  spelling := "<results> — <action>",
  body :=
    { results := results, action := action }
}
