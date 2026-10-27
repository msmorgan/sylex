/-- Relic Ward -/
card relicWard : Spelled :=
  { name := "Relic Ward", cost := [generic 1, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ asThoughFlashSacrificedAtCleanup,
        enchant isArtifact,
        static (abilityGrant (enchanted artifact) shroud) ] }
