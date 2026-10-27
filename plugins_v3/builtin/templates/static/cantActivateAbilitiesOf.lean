template cantActivateAbilitiesOf (p : Predicate) : StaticSpec := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := cant any (activate (allOf (and [abilityHead .anyActivated, abilityOf (allOf p)])))
}
