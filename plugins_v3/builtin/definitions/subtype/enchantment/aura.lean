/-- [CR#704.5m] An Aura that is not attached is put into its owner's graveyard. -/
subtype aura := {
  lexical := { lemma := "Aura" },
  spelling := "Aura",
  kind := .enchantment,
  name := "Aura",
  confers :=
    [.stateBased
        (not (isAttached thisAura))
        (actionMove thisAura
          (zone .battlefield none) (some graveyard)
          .arrival [])]
}
