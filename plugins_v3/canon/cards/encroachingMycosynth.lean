card := {
  name := "Encroaching Mycosynth",
  cost := [3, blue],
  types := [artifact],
  text := [
    qualityChange
      (and
         [
           allOf [isPermanent, non isLand, youControl],
           allOf
             [
               or [isArtifact, isBattle, isCreature, isEnchantment, isPlaneswalker],
               isSpell,
               youControl
             ],
           allOf [isCard, isPermanentCard, non isLand, youOwn, not (in_ battlefield)]
         ])
      adds (bundle { types := [artifact] })
  ]
}
