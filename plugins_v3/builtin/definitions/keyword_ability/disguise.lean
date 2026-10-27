keyword disguise (cost : Cost) : Ability := {
  lexical := { lemma := "disguise", parameters := [[Cost]] },
  spelling := "disguise <cost>",
  body :=
    [ static
        (can you (cast thisCard (payment := .faceDown faceDownWardedCreature (mana [generic 3])))),
      static (.specialAction none none cost (turnFaceUp this_)) ]
}
