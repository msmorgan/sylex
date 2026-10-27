/-- [CR#704.5m] An Aura that is not attached is put into its owner's graveyard. -/
subtype aura := {
  lexical := { lemma := "Aura" },
  spelling := "Aura",
  kind := .enchantment,
  name := "Aura",
  confers :=
    [Semantics.Conferral.stateBased
        (Semantics.Templates.Condition.not (Semantics.Templates.Condition.isAttached Semantics.Templates.NounPhrase.thisAura))
        (Semantics.Templates.Action.actionMove Semantics.Templates.NounPhrase.thisAura
          (Semantics.Templates.Location.zone Semantics.Zone.battlefield Option.none) (Option.some Semantics.Templates.Location.graveyard)
          Semantics.ObservationPoint.arrival [])]
}
