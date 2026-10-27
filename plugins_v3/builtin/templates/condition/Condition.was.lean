/-- "if <action> was done <within>", the passive with no agent. -/
template Condition.was (action : Action) (within : LookbackSpan) : Condition := {
  spelling := "if <action> was done <within>",
  body :=
    Semantics.Templates.Condition.conditionDid { subj := Semantics.Templates.Subject.any, action := action, within := Option.some within }
}
