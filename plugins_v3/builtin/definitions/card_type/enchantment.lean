/-- Enchantment: a permanent type [CR#110.4].

 [CR#303.1] An enchantment card may be cast from its owner's hand as a sorcery. -/
card_type enchantment := {
  lexical := { lemma := "enchantment" },
  spelling := "enchantment",
  name := "Enchantment",
  cardClass := some .permanentCard,
  stats := [],
  confers := [castableFrom yourHand asSorcery]
}
