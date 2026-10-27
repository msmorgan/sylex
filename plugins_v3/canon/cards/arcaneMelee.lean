card arcaneMelee : Spelled :=
  { name := "Arcane Melee", cost := [generic 4, pip .blue], types := [.enchantment],
    text := [static (spellsCost (and [isInstantOrSorcery, isSpell]) (less 2))] }
