/-- Animate Dead -/
card animateDead : Spelled :=
  { name := "Animate Dead", cost := [generic 1, pip .black], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant creatureCardInAGraveyard,
        when (does thisAura enter)
          (sequentially
            [ swapsEnchantForLinkedEntry,
              returnToBattlefield (enchanted (.ofType .card .creature)) [under you],
              attachTo thisAura it,
              sacrificedWhenItLeaves thisAura ])
          (if_ := is it (in_ battlefield)),
        static (getsPt (enchanted creature) (down 1) (up 0)) ] }
