keyword disguise (cost : Cost) : Ability := {
  lexical := { lemma := "disguise", parameters := [[Cost]] },
  spelling := "disguise <cost>",
  body :=
    [static
        (can (actor you)
          (cast thisCard none
            (playPaymentFaceDown faceDownWardedCreature
              (mana [generic 3])))),
      static
        (staticSpecSpecialAction none none cost
          (turnFaceUp this_))]
}
