/-- Guardian Beast -/
card := {
  name := "Guardian Beast",
  cost := [3, black],
  types := [creature],
  subtypes := [beast],
  text := [
    onlyWhile
      (conjunction
         [
           retention (cant any (attachTo (a aura) them)) (allOf [aura, attachedTo (those artifact)]),
           abilityGrant them indestructible,
           cant (bare [isPlayer, isOther]) (gainControlOf them)
         ]
         (bare [non isCreature, isArtifact, youControl]))
      (is thisCreature isUntapped)
  ],
  power := 2,
  toughness := 4
}
