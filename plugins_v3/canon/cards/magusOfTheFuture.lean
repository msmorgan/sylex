card magusOfTheFuture : Spelled :=
  { name := "Magus of the Future", cost := [generic 2, pip .blue, pip .blue, pip .blue],
    types := [.creature], subtypes := [human, wizard],
    text :=
      [ static (visibility .reveal you topOfLibrary),
        static playLandsAndCastSpellsFromTop ],
    power := stat 2, toughness := stat 3 }
