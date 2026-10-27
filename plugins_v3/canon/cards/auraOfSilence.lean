card auraOfSilence : Spelled :=
  { name := "Aura of Silence", cost := [generic 1, pip .white, pip .white],
    types := [.enchantment],
    text :=
      [ static (spellsCost (and [or [isArtifact, isEnchantment], isSpell,
          did yourOpponents (cast which) thisObject])
          (more 2)),
        activated (sacrifice thisEnchantment)
          (destroy (target (or [isArtifact, isEnchantment]))) ] }
