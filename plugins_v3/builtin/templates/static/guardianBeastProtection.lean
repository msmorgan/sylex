template guardianBeastProtection : StaticSpec := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := onlyWhile
    (conjunction
      [ retention (cant any (attachTo (a aura) them)) aurasAlreadyAttachedToThoseArtifacts,
        abilityGrant them indestructible,
        cant (bare (and [isPlayer, isOther])) (gainControlOf them) ]
      (bare (and [non isCreature, isArtifact, youControl])))
    (is thisCreature isUntapped)
}
