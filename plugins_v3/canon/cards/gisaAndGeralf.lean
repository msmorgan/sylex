card gisaAndGeralf : Spelled :=
  { name := "Gisa and Geralf", cost := [generic 2, pip .blue, pip .black],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, wizard],
    text :=
      [ when (does thisCreature enter) (mill 4),
        static (can you
                  (cast (counted (exactly 1) (and [isSpell, isCreature, zombie]))
                     (from_ := yourGraveyard)) (timing := onEachOfYourTurns)) ],
    power := stat 4, toughness := stat 4 }
