primitive doIf (condition : Condition) (action : Action)
    (otherwise : Option Action := none) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.doIf condition action otherwise
}
