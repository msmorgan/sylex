template atKnifepointOutlaws : Ability := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := static (partScope .turn you
            (abilityGrant (allOf (and [isOutlaw, youControl])) firstStrike))
}
