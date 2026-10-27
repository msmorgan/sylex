keyword transmute (cost : Cost) : Ability := {
  lexical := { lemma := "transmute", parameters := [[Cost]] },
  spelling := "transmute <cost>",
  body :=
    [ activatedOnlyDuring
        (.compound
          [ cost,
            .perform (discard this_) ])
        (sequentially
          [ search yourLibrary
              (a (and
                [ .isCard,
                  hasManaValue .eq
                    (.statOf (.stat .manaValue) (the (and [was (discard which), isCard]))) ])),
            reveal (that .card),
            putInto it hand,
            shuffle ])
        asSorcery ]
}
