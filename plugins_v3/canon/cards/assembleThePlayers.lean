card assembleThePlayers : Spelled :=
  { name := "Assemble the Players", cost := [generic 1, pip .white], types := [.enchantment],
    text :=
      [ static (visibility .lookAt you topOfLibrary),
        static (can you
                (cast (counted (exactly 1) (and [isSpell, isCreature, hasPower .atMost 2]))
                   (from_ := onTop)) (timing := eachTurn)) ] }
