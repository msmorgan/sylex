/-- Clone -/
card clone : Spelled :=
  { name := "Clone", cost := [generic 3, pip .blue], types := [.creature],
    subtypes := [shapeshifter],
    text := [static (enters thisCreature [asCopyOf true (a isCreature) []])],
    power := stat 0, toughness := stat 0 }
