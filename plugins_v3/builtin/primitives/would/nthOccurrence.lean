/-- "the <ordinal> time <clause> [each <per>]": the replaced event's ordinal, as "The first time
you would create one or more tokens each turn" [CR#614.1]. -/
-- [CR#614.1]
primitive nthOccurrence (ordinal : Ordinal) (clause : WouldClause)
    (per : Option Period := none) : WouldClause := {
  -- TODO(spelling): Context-dependent wording: the <ordinal> time <clause> [each <per>]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.WouldClause.nthOccurrence ordinal per clause
}
