template elspethsTalentGrant : StaticSpec := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := abilityGrant (enchanted planeswalker)
    (activated (loyaltySymbol (up 1))
      (create 3 (creatureToken 1 1 [.white] [soldier])))
}
