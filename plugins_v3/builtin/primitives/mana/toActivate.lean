/-- "to activate abilities [of <source>]" -/
primitive toActivate (source : Option Predicate) : SpendPurpose := {
  -- TODO(spelling): Include or omit the clauses supplied by source.
  body :=
    Semantics.Primitives.SpendPurpose.toActivate source
}
