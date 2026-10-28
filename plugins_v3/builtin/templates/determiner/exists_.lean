/-- "if there is a …", "if you control a …": the indefinite is published for the consequent
(`semantics-v3.md` §1.3, box 2). -/
template exists_ (p : Predicate) : Condition := {
  -- TODO(spelling): Context-dependent wording: if there is a …; string substitution cannot express its omitted or inflected parts.
  body :=
    thereAre (described (a .unmarked) p)
}
