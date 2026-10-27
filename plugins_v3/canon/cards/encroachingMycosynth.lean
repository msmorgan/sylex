card encroachingMycosynth : Spelled :=
  { name := "Encroaching Mycosynth", cost := [generic 3, pip .blue], types := [.artifact],
    text :=
      [ static (qualityChange (and [
            allOf (and [isPermanent, non isLand, youControl]),
            allOf (and
                     [or [isArtifact, isBattle, isCreature, isEnchantment, isPlaneswalker],
                      isSpell, youControl]),
            allOf (and [isCard, isPermanentCard, non isLand, youOwn, not (in_ battlefield)]) ])
              .adds
            (bundle { characteristics := { types := [.artifact] } })) ] }
