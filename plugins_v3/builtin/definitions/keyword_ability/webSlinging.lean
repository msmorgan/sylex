keyword webSlinging (cost : Cost) : Ability := {
  lexical := { lemma := "web-slinging", parameters := [[Cost]] },
  spelling := "web-slinging <cost>",
  body :=
    [ static
        (altCost this_
          (some
            (.compound
              [ cost,
                .perform (returnToHand (a (and [isCreature, isTapped, youControl]))) ]))) ]
}
