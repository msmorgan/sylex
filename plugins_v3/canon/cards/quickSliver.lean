card quickSliver : Spelled :=
  { name := "Quick Sliver", cost := [generic 1, pip .green], types := [.creature],
    subtypes := [sliver],
    text :=
      [ flash,
        static (can (a isPlayer) (cast (bare (and [isSpell, sliver])))
                  (asThough := asThoughHadFlash them)) ],
    power := stat 1, toughness := stat 1 }
