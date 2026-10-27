card bombur : Spelled :=
  { name := "Bombur, Gentle Dreamer", cost := [generic 2, pip .red],
    supertypes := [.legendary], types := [.creature],
    subtypes := [dwarf, bard],
    text :=
      [ storied,
        static (onlyUnless (doesntUntap thisCreature you)
          (is you (hasDesignation enduringStory))) ],
    power := stat 5, toughness := stat 3 }
