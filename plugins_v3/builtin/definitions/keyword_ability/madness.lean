keyword madness (cost : Cost) : Ability := {
  lexical := { lemma := "madness", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): madness [mana cost] / madness—[verbal cost]; separator and terminal period
  -- depend on the Cost structure.
  body :=
    [ static
        (replace (would (a isPlayer) (discard thisCard))
          (instead
            -- This escape crosses the nested discard scope, rather than wrapping a parameter.
            (.ownScope (some discard.deed) (exileFrom (.inCaller thisCard) hand)))),
      when
        (.causes (.event (does any (discard thisCard))) (does any (exileFrom thisCard hand)))
        (instruct (ownerOf thisCard)
          (mayOrElse
            (cast thisCard (from_ := exileZone) (payment := .payingInstead cost))
            (putInto thisCard graveyard))) ]
}
