/-- Recruit: draw a card, then discard a card; if you discarded a nonland card, create a 1/1 white
Human Soldier creature token. [CR#701.70a] -/
action recruit : Action := {
  lexical := { verbs := [{ lemma := "recruit", frames := [Intransitive] }] },
  spelling := "recruit",
  body :=
    sequentially
      [draw 1,
        discard
          (a (cardIn (handOf actor))),
        doIf
          (not
            (is
              (the
                (and
                  [isCard,
                    was (discard which)
                      (some thisWay)]))
              isLand))
          (create 1
            (written
              (creatureToken 1 1 [.white]
                [human, soldier])))]
}
