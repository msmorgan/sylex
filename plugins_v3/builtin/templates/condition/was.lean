/-- "if <action> was done <within>", the passive with no agent. -/
template was (action : Action) (within : LookbackSpan) : Condition := {
  spelling := "if <action> was done <within>",
  body :=
    conditionDid { subj := Templates.Subject.any, action := action, within := some within }
}
