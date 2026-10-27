template kitsuneMysticFlip : Ability := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := at_ (beginningOfEach .endStep)
    (if_ := is thisCreature
      (attachedBy (some .enchanted)
        (counted (orMore 2) aura)))
    (flip thisCreature)
}
