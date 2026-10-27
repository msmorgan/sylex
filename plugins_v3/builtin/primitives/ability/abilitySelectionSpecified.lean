primitive abilitySelectionSpecified (abilities : List Semantics.AbilityLost) : Semantics.AbilitySelection := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.AbilitySelection.specified abilities
}
