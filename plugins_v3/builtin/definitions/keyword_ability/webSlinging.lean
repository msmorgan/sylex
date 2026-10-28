/-- "Web-slinging [cost]" means "You may cast this spell by paying [cost] and returning a tapped
creature you control to its owner's hand rather than paying its mana cost." [CR#702.188a] -/
keyword webSlinging (cost : Cost) : Ability := {
  lexical := { lemma := "web-slinging", parameters := [[Cost]] },
  spelling := "web-slinging <cost>",
  body :=
    [static
        (altCost this_
          (some
            (compound
              [cost,
                perform
                  (returnToHand
                    (a
                      (and
                        [isCreature, isTapped,
                          youControl])))])))]
}
