template thisEnchantment : NounPhrase := {
  spelling := "this enchantment",
  body :=
    Semantics.Templates.NounPhrase.headed (Semantics.NounWord.type Semantics.CardType.enchantment) Semantics.Templates.NounPhrase.this_
}
