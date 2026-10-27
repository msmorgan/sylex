card secretsOfTheDead : Spelled :=
  { name := "Secrets of the Dead", cost := [generic 2, pip .blue], types := [.enchantment],
    text :=
      [ whenever (does you (cast (a
        (and [isSpell, did any (cast which (from_ := yourGraveyard)) thisObject]))))
          (draw 1) ] }
