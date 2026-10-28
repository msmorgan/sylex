/-- Flow State: "Look at the top three cards of your library. Put one of them into your hand and
the rest on the bottom of your library in any order. If there is an instant card and a sorcery
card in your graveyard, instead put two of them into your hand and the rest on the bottom of
your library in any order." A self-replacement [CR#614.15]: the replacement reads what the
replaced action read, the cards looked at, not the part and remainder it would have made. The
domain its existential condition publishes ranks below those cards (owner ruling 2026-10-22,
batch 4), so "them" and "the rest" read the cards looked at. -/
card := {
  name := "Flow State",
  cost := [1, blue],
  types := [sorcery],
  text := [
    spell
      [
        lookAt (topSlice 3),
        replace
          [
            putInto (someOf (exactly 1) them) hand,
            putInto theRest onBottom (riders := [(inOrder anyOrder)])
          ]
          (doIf
             (thereAre
                (both (a [isInstant, cardIn yourGraveyard]) (a [isSorcery, cardIn yourGraveyard])))
             (instead
                [
                  putInto (someOf (exactly 2) them) hand,
                  putInto theRest onBottom (riders := [(inOrder anyOrder)])
                ]))
      ]
  ]
}
