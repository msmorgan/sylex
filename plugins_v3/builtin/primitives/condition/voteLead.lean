primitive voteLead (label : String) (orTied : Bool) : Condition := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Condition.voteLead label orTied
}
