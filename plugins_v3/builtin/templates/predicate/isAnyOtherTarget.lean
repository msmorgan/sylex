/-- "any other target" -/
template isAnyOtherTarget : Predicate := {

  -- TODO(spelling): any other target; the predicate supplies eligibility and exclusion while the
  -- enclosing noun phrase supplies any/target.
  body :=
    Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isAnyTarget, Semantics.Templates.Predicate.isOther]
}
