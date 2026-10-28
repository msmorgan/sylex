/-- Bewitching Leechcraft -/
card := {
  name := "Bewitching Leechcraft",
  cost := [1, blue],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    when (does thisAura enter) (tap (enchanted creature)),
    abilityGrant (enchanted creature)
      (static
         (replace (would thisCreature (setStatus untapped theActor))
            (instead
               (doIfDone (removeCounters (exactly 1) p1p1Counter thisCreature) (untap thisCreature)))
            (duringPart untapStep you)))
  ]
}
