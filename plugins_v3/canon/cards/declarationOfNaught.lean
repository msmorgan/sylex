card declarationOfNaught : Spelled :=
  { name := "Declaration of Naught", cost := [pip .blue, pip .blue], types := [.enchantment],
    text :=
      [ static (entersChoosing thisEnchantment .cardName),
        activated [pip .blue] (counter (target (and [isSpell, named chosenName]))) ] }
