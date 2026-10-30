/-- "Eternalize [cost]" means "[Cost], Exile this card from your graveyard: Create a token that's a
copy of this card, except it's black, it's 4/4, it has no mana cost, and it's a Zombie in
addition to its other types. Activate only as a sorcery." [CR#702.129a] -/
keyword eternalize (cost : Cost) : Ability := {
  lexical := { lemma := "eternalize", parameters := [[Cost]] },
  spelling := "eternalize <cost>",
  body :=
    [activatedOnlyDuring
        (compound
          [cost,
            perform
              (exileFrom thisCard yourGraveyard)])
        (create 1
          (copyOf this_
            [characteristicEdits
                [colors sets
                    (.some [black]),
                  stat power (set 4),
                  stat toughness (set 4),
                  manaCost none,
                  typeLine adds
                    { subtypes := some [zombie] }]]))
        asSorcery]
}
