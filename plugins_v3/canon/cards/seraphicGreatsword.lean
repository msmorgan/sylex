/-- Seraphic Greatsword -/
card seraphicGreatsword : Spelled :=
  { name := "Seraphic Greatsword", cost := [generic 1, pip .white], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (getsPt (equipped creature) (up 2) (up 2)),
        whenever
          (does (equipped creature)
             (attack
                   (a (and [isPlayer, superlative .max (.playerStat .lifeTotal) isPlayer]))))
          (createTappedAttacking 1
            { characteristics :=
              { colors := [.white], types := [.creature], subtypes := [angel],
                text := [flying], power := stat 4, toughness := stat 4 } }
            (some (that player))),
        equip [generic 4] ] }
