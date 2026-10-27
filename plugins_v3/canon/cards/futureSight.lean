card futureSight : Spelled :=
  { name := "Future Sight", cost := [generic 2, pip .blue, pip .blue, pip .blue],
    types := [.enchantment],
    text :=
      [ static (visibility .reveal you topOfLibrary),
        static (conjunction [(can you (play (allOf isLand) (from_ := onTop))), (can you (cast (bare
          isSpell) (from_ := onTop)))]) ] }
