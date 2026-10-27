card nyleasPresence : Spelled :=
  { name := "Nylea's Presence", cost := [generic 1, pip .green], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isLand,
        when (does thisAura enter) (draw 1),
        static (qualityChange (enchanted land) .adds (everyTypeOf
            .basicLand)) ] }
