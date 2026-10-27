card ingeniousArtillerist : Spelled :=
  { name := "Ingenious Artillerist", cost := [generic 2, pip .red], types := [.creature],
    subtypes := [human, artificer],
    text :=
      [ whenever (does (counted (orMore 1) (and [isArtifact, youControl])) enter)
          (instruct thisCreature (dealDamage groupSize eachOpponent)) ],
    power := stat 3, toughness := stat 1 }
