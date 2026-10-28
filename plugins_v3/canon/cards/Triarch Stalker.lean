card := {
  name := "Triarch Stalker",
  cost := [3, black, black],
  types := [artifact, creature],
  subtypes := [necron],
  text := [
    flavorWord "Targeting Relay" (at_ (beginningOfYour combat) (choose anOpponent)),
    abilityGrant (allOf [isCreature, inCombat attackerOf (the isChosenPlayer)]) menace
  ],
  power := 4,
  toughness := 5
}
