/-- Rune Snag -/
card runeSnag : Spelled :=
  { name := "Rune Snag", cost := [generic 1, pip .blue], types := [.instant],
    text :=
      [ spell (instruct (controllerOf (target isSpell)) (doUnless (instruct you (counter it))
          (compound
            [ [generic 2],
              scaledGenericMana
                (forEach 2
                   (and [named "Rune Snag", in_ graveyard])) ]))) ] }
