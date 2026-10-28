/-- "A creature with skulk can't be blocked by creatures with greater power." [CR#702.118b] -/
keyword skulk : Ability := {
  lexical := { lemma := "skulk" },
  spelling := "skulk",
  body :=
    [static
        (cant
          (by_
            (allOf
              (and
                [isCreature,
                  hasPower greater
                    (powerOf thisCreature)])))
          (block thisCreature))]
}
