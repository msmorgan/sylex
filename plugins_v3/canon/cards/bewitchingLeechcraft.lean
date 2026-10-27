/-- Bewitching Leechcraft -/
card bewitchingLeechcraft : Spelled :=
  { name := "Bewitching Leechcraft", cost := [generic 1, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        when (does thisAura enter) (tap (enchanted creature)),
        static (abilityGrant (enchanted creature)
          (static (replace (would thisCreature (setStatus untapped theActor)) (instead
              (doIfDone
                (removeCounters (exactly 1) p1p1Counter thisCreature)
                (untap thisCreature))) (duringPart .untapStep you)))) ] }
