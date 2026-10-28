keyword soulbond : Ability := {
  lexical := { lemma := "soulbond" },
  spelling := "soulbond",
  body :=
    [when
        (does (actor thisCreature) enter)
        (may
          (relate thisCreature paired
            (a
              (and
                [and
                    [isCreature, youControl, isOther],
                  not
                    (related paired none)]))))
        [] [] none none none
        (some
          (and
            [is thisCreature
                (and
                  [youControl,
                    not
                      (related paired none)]),
              exists_
                (and
                  [and
                      [isCreature, youControl, isOther],
                    not
                      (related paired none)])])),
      whenever
        (does
          (actor
            (a
              (and
                [isCreature, youControl, isOther])))
          enter)
        (may
          (relate (that creature)
            paired thisCreature))
        [] [] none none none
        (some
          (and
            [is thisCreature
                (and
                  [youControl,
                    not
                      (related paired none)]),
              is (that creature)
                (and
                  [youControl,
                    not
                      (related paired none)])]))]
}
