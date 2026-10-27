/-- Guardian Beast -/
card guardianBeast : Spelled :=
  { name := "Guardian Beast", cost := [generic 3, pip .black], types := [.creature],
    subtypes := [beast],
    text :=
      [ static (onlyWhile
            (conjunction
              [ retention (cant any (attachTo (a aura) them)) (allOf (and [aura, attachedTo (those
                artifact)])),
                abilityGrant them indestructible,
                cant (bare (and [isPlayer, isOther])) (gainControlOf them) ]
              (bare (and [non isCreature, isArtifact, youControl])))
            (is thisCreature isUntapped)) ],
    power := stat 2, toughness := stat 4 }
