card youngPyromancer : Spelled :=
  { name := "Young Pyromancer", cost := [generic 1, pip .red], types := [.creature],
    subtypes := [human, shaman],
    text :=
      [ whenever (does you (cast (a
        (and [isInstantOrSorcery, isSpell]))))
          (create 1 (creatureToken 1 1 [.red] [elemental]))
                ],
    power := stat 2, toughness := stat 1 }
