/-- "Soulbond" means "When this creature enters, if you control both this creature and another
creature and both are unpaired, you may pair this creature with another unpaired creature you
control for as long as both remain creatures on the battlefield under your control" and
"Whenever another creature you control enters, if you control both that creature and this one
and both are unpaired, you may pair that creature with this creature for as long as both remain
creatures on the battlefield under your control." [CR#702.95a] -/
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
