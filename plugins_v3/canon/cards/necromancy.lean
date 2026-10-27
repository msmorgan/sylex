/-- Necromancy -/
card necromancy : Spelled :=
  { name := "Necromancy", cost := [generic 2, pip .black], types := [.enchantment],
    text :=
      [ asThoughFlashSacrificedAtCleanup,
        when (does thisEnchantment enter)
          (sequentially
            [ establish (conjunction
                [ addSubtype it aura,
                  abilityGrant it (enchant (creaturePutOntoTheBattlefieldWith this_)) ]
                (subject := it)),
              putOntoBattlefieldUnderYourControl (target creatureCardInAGraveyard),
              attachTo thisEnchantment it,
              sacrificedWhenItLeaves thisEnchantment ])
          (if_ := is it (in_ battlefield)) ] }
