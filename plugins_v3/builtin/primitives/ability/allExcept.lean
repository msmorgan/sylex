primitive allExcept (except : Option Predicate) : AbilitySelection := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.AbilitySelection.allExcept except
}
