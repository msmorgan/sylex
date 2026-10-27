card vexingSphinx : Spelled :=
  { name := "Vexing Sphinx", cost := [generic 1, pip .blue, pip .blue], types := [.creature],
    subtypes := [sphinx],
    text :=
      [ flying,
        cumulativeUpkeep (discard (a (cardIn yourHand))),
        when (does thisCreature die) (draw (countersOn ageCounter it)) ],
    power := stat 4, toughness := stat 4 }
