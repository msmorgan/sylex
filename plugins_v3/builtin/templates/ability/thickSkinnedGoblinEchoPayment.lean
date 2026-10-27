template thickSkinnedGoblinEchoPayment : Ability := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := static (replace
    (would you (pay anyCost .once .paid (allOf (and [isPermanent, youControl]))
      (some echo)))
    (instead (may (pay [generic 0] .once))))
}
