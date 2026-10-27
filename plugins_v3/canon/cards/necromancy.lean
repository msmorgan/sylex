/-- Necromancy -/
card necromancy : Spelled :=
  { name := "Necromancy", cost := [generic 2, pip .black], types := [.enchantment],
    text :=
      [ asThoughFlashSacrificedAtCleanup,
        when (does thisEnchantment enter)
          (sequentially
            [ establish (conjunction
                [ addSubtype it aura,
                  abilityGrant it (enchant ((and [isCreature, did (by_ this_) (putOntoBattlefield
                    which)]))) ]
                (subject := it)),
              putOntoBattlefieldUnderYourControl (target (and [isCreature, isCard, in_ graveyard])),
              attachTo thisEnchantment it,
              (delay (does thisEnchantment (leave battlefield)) (instruct (controllerOf (that
                creature)) (sacrifice it))) ])
          (if_ := is it (in_ battlefield)) ] }
