/-- Compare two amounts. -/
primitive compareAmt (subject : Amount) (comparator : Comparator) (bound : Amount) :
    Condition := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Condition.compareAmt subject comparator bound
}
