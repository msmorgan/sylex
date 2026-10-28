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
            [edits
                [colors .sets
                    (.some [.black]),
                  stat .sets .power 4,
                  stat .sets .toughness 4,
                  manaCost none,
                  typeLine .adds
                    { subtypes := some [zombie] }]]))
        asSorcery]
}
