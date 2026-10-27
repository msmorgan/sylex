/-- "if <clause> or <clause> would happen": any of several events [CR#614.1]. -/
-- [CR#614.1]
primitive wouldAny (clauses : List WouldClause) : WouldClause := {
  -- TODO(spelling): Context-dependent wording: if <clause> or <clause> would happen; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.WouldClause.wouldAny clauses
}
