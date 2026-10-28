/-- Compare two amounts. -/
primitive compareAmt (subject : Amount) (comparator : Comparator) (bound : Amount) :
    Condition := {
  spelling := "<subject> is <comparator> <bound>",
  body :=
    Semantics.Primitives.Condition.compareAmt subject comparator bound
}
