/-- Keeper of the Lens -/
card keeperOfTheLens : Spelled :=
  { name := "Keeper of the Lens", cost := [generic 1], types := [.artifact, .creature],
    subtypes := [golem],
    text :=
      [ static (visibility .lookAt you
          (objects (allOf (and [isCreature, isFaceDown, youDontControl])))) ],
    power := stat 1, toughness := stat 2 }
