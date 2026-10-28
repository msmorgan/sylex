keyword eternalize (cost : Cost) : Ability := {
  lexical := { lemma := "eternalize", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): eternalize [mana cost] / eternalize—[verbal cost]; separator and terminal
  -- period depend on the Cost structure.
  body :=
    [activatedOnlyDuring
        (compound
          [cost,
            perform
              (exileFrom thisCard yourGraveyard)])
        (create 1
          (copyOf this_
            [copyExceptEdits
                [characteristicEditColors .sets
                    (.some [.black]),
                  characteristicEditStat .sets .power 4,
                  characteristicEditStat .sets .toughness 4,
                  characteristicEditManaCost none,
                  characteristicEditTypeLine .adds
                    { subtypes := some [zombie] }]]))
        asSorcery]
}
