/-- Necromancy -/
card := {
  name := "Necromancy",
  cost := [2, black],
  types := [enchantment],
  text := [
    asThoughFlashSacrificedAtCleanup,
    when (does thisEnchantment enter)
      [
        establish
          (conjunction
             [
               addSubtype it aura,
               abilityGrant it (enchant [isCreature, did (by_ this_) (putOntoBattlefield which)])
             ]
             (subject := it)),
        putOntoBattlefieldUnderYourControl (target [isCreature, isCard, in_ graveyard]),
        attachTo thisEnchantment it,
        (delay (does thisEnchantment (leave battlefield))
           (instruct (controllerOf (that creature)) (sacrifice it)))
      ]
      (if_ := is it (in_ battlefield))
  ]
}
