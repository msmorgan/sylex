/-- "if <subj> <action>ed <within>": a condition always reads a written span. -/
primitive did (subj : Subject) (action : Action) (within : LookbackSpan) :
    Condition := {
  body :=
    Semantics.Primitives.Condition.did { subj := subj, action := action, within := some within }
}
