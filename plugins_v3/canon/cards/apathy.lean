/-- Apathy -/
card apathy : Spelled :=
  { name := "Apathy", cost := [pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (doesntUntap (enchanted creature)
          (controllerOf (enchanted creature))),
        at_ (beginningOfPossessed .upkeep (controllerOf (enchanted creature)))
          (instruct (that player) (may (discard (aAtRandom (cardIn (handOf (that player)))))
            (untap (that creature)))) ] }
