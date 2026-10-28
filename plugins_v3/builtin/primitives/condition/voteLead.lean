/-- Test whether the named choice has the most votes, optionally allowing a tie. -/
primitive voteLead (label : String) (orTied : Bool) : Condition := {
  body :=
    Semantics.Primitives.Condition.voteLead label orTied
}
