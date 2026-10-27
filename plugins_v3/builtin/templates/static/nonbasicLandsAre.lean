template nonbasicLandsAre (type : Subtype) : StaticSpec := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := qualityChange (allOf (and [isLand, non (hasSupertype .basic)])) .sets
    (bundle { characteristics := { subtypes := [type] } })
}
