/-- "<subj> <action>s": an action event, in the voice `subj` carries. -/
primitive does (subj : Subject) (action : Action) : GameEvent := {
  body :=
    Semantics.Primitives.GameEvent.does subj action
}
