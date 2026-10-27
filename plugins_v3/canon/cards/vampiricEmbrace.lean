/-- Vampiric Embrace. -/
card vampiricEmbrace : Spelled :=
  { name := "Vampiric Embrace", cost := [generic 2, pip .black, pip .black],
    types := [.enchantment], subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (getsPt
          (enchanted creature)
          (up 2) (up 2)),
        static (abilityGrant
          (enchanted creature) flying),
        whenever (does (creatureDamagedByAttachedHostThisTurn .enchanted) die)
          (putCounters 1 p1p1Counter
            (that creature)) ] }
