template phyrexianIngesterPump : Ability := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := static (conjunction [ getsPt thisCreature (up (letter .x)) (up (letter .y)),
      letterDefinition .x
        (powerOf (the (and [isCreature, in_ (exiledWith thisCreature)]))),
      letterDefinition .y (toughnessOf (that card)) ])
}
