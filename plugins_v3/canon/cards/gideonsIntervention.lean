/-- Gideon's Intervention -/
card gideonsIntervention : Spelled :=
  { name := "Gideon's Intervention", cost := [generic 2, pip .white, pip .white],
    types := [.enchantment],
    text :=
      [ static (entersChoosing thisEnchantment .cardName),
        static (cant yourOpponents (cast (bare (and [isSpell, named chosenName])))),
        static
          (replace
            (would (by_ (allOf (and [isSource, named chosenName])))
              (dealDamage anyAmount (youAnd (allOf (and [isPermanent, youControl])))))
            (instead (preventDamage allDamage))) ] }
