/-- [CR#704.5m] An Aura that is not attached is put into its owner's graveyard. -/
subtype aura := {
  lexical := { lemma := "Aura" },
  spelling := "Aura",
  kind := .enchantment,
  name := "Aura",
  confers :=
    [Semantics.Conferral.stateBased
        (not (isAttached thisAura))
        (actionMove thisAura
          (zone Semantics.Zone.battlefield none) (some graveyard)
          Semantics.ObservationPoint.arrival [])]
}
