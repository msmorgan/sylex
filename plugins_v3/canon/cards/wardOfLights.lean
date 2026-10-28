/-- Ward of Lights: its protection doesn't remove it, as Flickering Ward's doesn't
[CR#702.16n]. -/
card := {
  name := "Ward of Lights",
  cost := [white, white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    asThoughFlashSacrificedAtCleanup,
    enchant isCreature,
    entersChoosing thisAura color,
    retention (abilityGrant (enchanted creature) (protection (ofChosen color))) thisAura
  ]
}
