/-- Soul Ransom -/
card soulRansom : Spelled :=
  { name := "Soul Ransom", cost := [generic 2, pip .blue, pip .black], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (controlGrant you (enchanted creature)),
        activatedBy
          (repeatTimes 2
            (sequentially [choose (a (cardIn hand)), discard (that card)]))
          (sequentially [instruct (controllerOf thisAura) (sacrifice it), instruct they (draw 2)])
          yourOpponents ] }
