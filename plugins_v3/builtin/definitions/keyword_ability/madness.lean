/-- "Madness [cost]" means "If a player would discard this card, that player discards it, but exiles
it instead of putting it into their graveyard" and "When this card is exiled this way, its owner
may cast it by paying [cost] rather than paying its mana cost. If that player doesn't, they put
this card into their graveyard." [CR#702.35a] -/
keyword madness (cost : Cost) : Ability := {
  lexical := { lemma := "madness", parameters := [[Cost], [DashNonManaCost]] },
  spelling := "madness <cost>",
  body :=
    [static
        (replace
          (would (actor (a isPlayer))
            (discard thisCard))
          (instead
            -- This escape crosses the nested discard scope, rather than wrapping a parameter.
            (.ownScope (some Semantics.Definitions.discard.deed)
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
