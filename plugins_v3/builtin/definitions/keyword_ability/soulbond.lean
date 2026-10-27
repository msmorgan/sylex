keyword soulbond : Ability := {
  lexical := { lemma := "soulbond" },
  spelling := "soulbond",
  body :=
    [ when (does thisCreature enter)
        (if_ := some (and
          [ is thisCreature (and [youControl, not (.related paired none)]),
            exists_
              (and
                [ and [isCreature, youControl, isOther],
                  not (.related paired none) ]) ]))
        (may
          (.relate thisCreature paired
            (a
              (and
                [ and [isCreature, youControl, isOther],
                  not (.related paired none) ])))),
      whenever
        (does (a (and [isCreature, youControl, isOther])) enter)
        (if_ := some (and
          [ is thisCreature (and [youControl, not (.related paired none)]),
            is (that creature) (and [youControl, not (.related paired none)]) ]))
        (may (.relate (that creature) paired thisCreature)) ]
}
