template loyaltyAbilityOfEnchanted : NounPhrase := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := a (and [abilityHead .loyalty, abilityOf (enchanted planeswalker)])
}
