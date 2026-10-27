/-- Chromatic Armor -/
card chromaticArmor : Spelled :=
  { name := "Chromatic Armor", cost := [generic 1, pip .white, pip .blue],
    types := [.enchantment], subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (entersChoosing thisAura .color),
        static (entersWithCounters thisAura 1 sleightCounter),
        static
          (replace
            (would (by_ (allOf (and [isSource, ofTheLastChosen .color])))
              (dealDamage anyAmount (enchanted creature)))
            (instead (preventDamage allDamage))),
        activated (mana [.variable])
          (sequentially
            [ putCounters 1 sleightCounter thisAura,
              choose (a (quality .color)),
              define .x (countersOn sleightCounter thisAura) ]) ] }
