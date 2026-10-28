/-- "Embalm [cost]" means "[Cost], Exile this card from your graveyard: Create a token that's a copy
of this card, except it's white, it has no mana cost, and it's a Zombie in addition to its other
types. Activate only as a sorcery." [CR#702.128a] -/
keyword embalm (cost : Cost) : Ability := {
  lexical := {
    lemma := "embalm",
    parameters := [[Cost]],
    verbs := [{ lemma := "embalm", frames := [Monotransitive] }]
  },
  spelling := "embalm <cost>",
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
                    (.some [.white]),
                  manaCost none,
                  typeLine .adds
                    { subtypes := some [zombie] }]]))
        asSorcery]
}
