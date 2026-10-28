/-- "Mutate [cost]": "You may pay [cost] rather than pay this spell's mana cost. If you do, it
becomes a mutating creature spell and targets a non-Human creature with the same owner as this
spell" [CR#702.140a]. One "if you do" gates the added target [CR#601.2c] and the merge alike:
the mutate cost's choice was taken. -/
keyword mutate (cost : Cost) : Ability := {
  lexical := {
    lemma := "mutate",
    parameters := [[Cost]],
    verbs := [{ lemma := "mutate", frames := [Intransitive] }]
  },
  spelling := "mutate <cost>",
  body :=
    [static (altCost this_ (some cost)),
      static
        (onlyWhile
          (addsTarget thisSpell
            (and
              [isCreature,
                not (hasSubtype human),
                ownedBy (ownerOf thisSpell)]))
          (choiceWasTaken (.byKeyword mutate.label)
            thisSpell)),
      static
        (replace
          (would (actor thisSpell) enter)
          (doIf
            (choiceWasTaken (.byKeyword mutate.label)
              thisSpell)
            (instead
              (chooseOne
                (merge thisSpell
                  (the
                    (was (target which)))
                  top)
                (merge thisSpell
                  (the
                    (was (target which)))
                  bottom))))
          none repeatedly none)]
}
