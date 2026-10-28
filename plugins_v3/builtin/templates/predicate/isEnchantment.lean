template isEnchantment : Predicate := {
  spelling := "enchantment",
  body :=
    hasType Semantics.CardType.enchantment
}
