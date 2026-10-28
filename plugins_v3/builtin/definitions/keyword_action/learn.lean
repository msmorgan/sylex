/-- "learn" (`Definition`, [CR#701.48a]): "You may discard a card. If you do, draw a card. If you
didn't discard a card, you may reveal a Lesson card you own from outside the game and put it into
your hand." The second condition is the discard's result, not the choice: a player who chose to
discard with no card in hand discarded none. -/
action learn : Action := {
  lexical := { verbs := [{ lemma := "learn", frames := [Intransitive] }] },
  spelling := "learn",
  body :=
    sequentially
      [may (discard (a isCard))
          (some (draw 1)),
        doIf
          (not
            (exists_
              (and
                [isCard,
                  was (discard which)
                    (some thisWay)])))
          (may
            (sequentially
              [reveal
                  (a
                    (and
                      [isCard, hasSubtype lesson,
                        ownedBy actor, outsideTheGame])),
                putInto it
                  (handOf actor)]))]
}
