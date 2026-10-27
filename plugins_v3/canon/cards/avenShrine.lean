card avenShrine : Spelled :=
  { name := "Aven Shrine", cost := [generic 1, pip .white, pip .white],
    types := [.enchantment],
    text :=
      [ whenever (does (a isPlayer) (cast (a isSpell)))
          (sequentially
            [ instruct they (gainLife (letter .x)),
              define .x (countOf (and [in_ graveyard, (shares .cardName .share (that .spell))])) ]) ] }
