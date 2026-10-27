/-- Triarch Stalker -/
card triarchStalker : Spelled :=
  { name := "Triarch Stalker", cost := [generic 3, pip .black, pip .black],
    types := [.artifact, .creature], subtypes := [necron],
    text :=
      [ flavorWord "Targeting Relay"
          (at_ (beginningOfYour .combat) (choose anOpponent)),
        static (abilityGrant (allOf (and [isCreature, inCombat .attackerOf (the
            isChosenPlayer)]))
          menace) ],
    power := stat 4, toughness := stat 5 }
