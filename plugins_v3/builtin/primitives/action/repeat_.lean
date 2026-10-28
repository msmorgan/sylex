/-- Repeat according to the specified repetition. -/
primitive repeat_ (repetition : Repetition) : Action := {
  body :=
    Semantics.Primitives.Action.repeat_ repetition
}
