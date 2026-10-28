keyword disguise (cost : Cost) : Ability := {
  lexical := { lemma := "disguise", parameters := [[Cost]] },
  spelling := "disguise <cost>",
  body :=
    [static
        (can (actor you)
          (cast thisCard none
            (faceDown faceDownWardedCreature
              (mana [generic 3])))),
      static
        (specialAction none cost (turnFaceUp this_) none)]
}
