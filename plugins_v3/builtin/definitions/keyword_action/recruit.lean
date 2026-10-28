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
              (creatureToken 1 1 [Semantics.Color.white]
                [human, soldier])))]
}
