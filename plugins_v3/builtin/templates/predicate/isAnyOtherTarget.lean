/-- "any other target" -/
template isAnyOtherTarget : Predicate := {
  -- TODO(spelling): any other target; the predicate supplies eligibility and exclusion while the
  -- enclosing noun phrase supplies any/target.
  body :=
    and [isAnyTarget, .other]
}
