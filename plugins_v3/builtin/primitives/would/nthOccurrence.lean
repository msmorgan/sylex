/-- "the <ordinal> time <clause> [each <per>]": the replaced event's ordinal, as "The first time
you would create one or more tokens each turn" [CR#614.1]. -/
-- [CR#614.1]
primitive nthOccurrence (ordinal : Ordinal) (clause : WouldClause)
    (per : Option Period := none) : WouldClause := {
  -- TODO(spelling): Include or omit the clauses supplied by per.
  body :=
    Semantics.Primitives.WouldClause.nthOccurrence ordinal per clause
}
