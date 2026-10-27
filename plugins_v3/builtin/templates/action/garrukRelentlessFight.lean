template garrukRelentlessFight : Action := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := sequentially
    [ instruct thisPlaneswalker (dealDamage 3 (target isCreature)),
      dealDamageOwnPower (that creature) thisPlaneswalker ]
}
