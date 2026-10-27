card aggressiveMining : Spelled :=
  { name := "Aggressive Mining", cost := [generic 3, pip .red], types := [.enchantment],
    text :=
      [ static (cant you (play (allOf isLand))),
        activatedOnlyOnce (sacrifice (a isLand)) (draw 2) oncePerTurn ] }
