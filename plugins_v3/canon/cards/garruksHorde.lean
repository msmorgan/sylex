card garruksHorde : Spelled :=
  { name := "Garruk's Horde", cost := [generic 5, pip .green, pip .green], types := [.creature],
    subtypes := [beast],
    text :=
      [ trample,
        static (visibility .reveal you topOfLibrary),
        static (can you (cast (bare (and [isSpell, isCreature])) (from_ := onTop))) ],
    power := stat 7, toughness := stat 7 }
