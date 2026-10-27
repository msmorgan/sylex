card labyrinthOfSkophos : Spelled :=
  { name := "Labyrinth of Skophos", types := [.land],
    text :=
      [ activated tapSymbol (addMana 1 (manaOf [[.colorless]])),
        activated (compound [[generic 4], tapSymbol])
          (removeFromCombat (target (and [isCreature, or [isAttacking, isBlocking]]))) ] }
