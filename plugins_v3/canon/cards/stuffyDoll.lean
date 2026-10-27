/-- Stuffy Doll -/
card stuffyDoll : Spelled :=
  { name := "Stuffy Doll", cost := [generic 5], types := [.artifact, .creature],
    subtypes := [construct],
    text :=
      [ indestructible,
        static (entersChoosingPlayer thisCreature (a isPlayer)),
        whenever (does any (dealDamage (to := thisCreature))) (instruct it (dealDamage thatMuch (the isChosenPlayer))),
        activated tapSymbol (instruct thisCreature (dealDamage 1 thisCreature)) ],
    power := stat 0, toughness := stat 1 }
