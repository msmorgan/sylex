card highGround : Spelled :=
  { name := "High Ground", cost := [pip .white], types := [.enchantment],
    text :=
        [static
           (can (each (and [isCreature, youControl]))
              (block (counted (additional (exactly 1)) isCreature))
        (timing := eachCombat))] }
