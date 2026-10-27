/-- "permanent card": a card that could be put onto the battlefield, one with an artifact,
battle, creature, enchantment, land, or planeswalker type [CR#110.4a]; a reading of the card's
types, wherever it is. -/
template isPermanentCard : Predicate := {
  spelling := "permanent card",
  body :=
    .and [.isCard, isPermanentType]
}
