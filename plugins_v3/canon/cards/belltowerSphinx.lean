card belltowerSphinx : Spelled :=
  { name := "Belltower Sphinx", cost := [generic 4, pip .blue], types := [.creature],
    subtypes := [sphinx],
    text :=
      [ flying,
        whenever (does (a isSource) (dealDamage anyAmount thisCreature))
          (instruct (controllerOf thatSource) (mill thatMuch)) ],
    power := stat 2, toughness := stat 5 }
