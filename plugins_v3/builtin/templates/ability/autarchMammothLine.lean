template autarchMammothLine : Ability := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := when (does thisCreature enter)
    (joins := [ joinedHeadWhile (does thisCreature attack)
        (whileTrue (is thisCreature (hasDesignation saddled))) ])
    (create 1 (creatureToken 3 3 [.green] [elephant]))
}
