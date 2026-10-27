card daydream : Spelled :=
  { name := "Daydream", cost := [pip .white], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ exile (target (and [isCreature, youControl])),
            returnToBattlefieldWithCounters (that card) (ownerOf (that card)) 1
              p1p1Counter ]),
        flashback [generic 2, pip .white] ] }
