primitive abilitySelectionAllExcept (except : Option Semantics.Predicate) : Semantics.AbilitySelection := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.AbilitySelection.allExcept except
}
