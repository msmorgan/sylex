template juxtaposeTie : Action := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := doIf (thereAre (counted (orMore 2)
          (and [isPermanent, controlledBy (a isPlayer),
                tiedFor .max (.stat .manaValue) (and [isPermanent, controlledBy (that player)])])))
    (instruct (controllerOf them) (choose (someOf (exactly 1) them)))
}
