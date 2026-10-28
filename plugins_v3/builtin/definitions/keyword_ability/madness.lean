keyword madness (cost : Cost) : Ability := {
  lexical := { lemma := "madness", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): madness [mana cost] / madness—[verbal cost]; separator and terminal period
  -- depend on the Cost structure.
  body :=
    [static
        (replace
          (would (actor (a isPlayer))
            (discard thisCard))
          (instead
            -- This escape crosses the nested discard scope, rather than wrapping a parameter.
            (.ownScope (some Semantics.Definitions.Action.discard.deed)
              (exileFrom (.inCaller thisCard)
                hand)))),
      when
        (causes
          (byEvent
            (does any
              (discard thisCard)))
          (does any
            (exileFrom thisCard hand)))
        (instruct (ownerOf thisCard)
          (mayOrElse
            (cast thisCard (some exileZone)
              (payingInstead cost))
            (putInto thisCard graveyard)))]
}
