card deadeyeBrawler : Spelled :=
  { name := "Deadeye Brawler", cost := [generic 2, pip .blue, pip .black],
    types := [.creature], subtypes := [human, pirate],
    text :=
      [ deathtouch,
        ascend,
        whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
          (if_ := is you (hasDesignation citysBlessing))
          (draw 1) ],
    power := stat 2, toughness := stat 4 }
