template isEnchantment : Predicate := {
  spelling := "enchantment",
  body :=
    Semantics.Templates.Predicate.hasType Semantics.CardType.enchantment
}
