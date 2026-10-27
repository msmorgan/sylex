card flourishingDefenses : Spelled :=
  { name := "Flourishing Defenses", cost := [generic 4, pip .green], types := [.enchantment],
    text :=
      [ whenever (does any (putCounters 1 m1m1Counter (a isCreature)))
          (may
            (create 1 (creatureToken 1 1 [.green] [elf, warrior]))) ] }
