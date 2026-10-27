template heartOfBogardanHeader : GameEvent := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := does (a isPlayer)
      (pay anyCost .once .unpaid (some thisEnchantment) (some cumulativeUpkeep))
}
