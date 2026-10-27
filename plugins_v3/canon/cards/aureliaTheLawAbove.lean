/-- Aurelia, the Law Above -/
card aureliaTheLawAbove : Spelled :=
  { name := "Aurelia, the Law Above", cost := [generic 3, pip .red, pip .white],
    supertypes := [.legendary], types := [.creature], subtypes := [angel],
    text :=
      [ flying,
        vigilance,
        haste,
        whenever (does (a isPlayer) (attackWith (counted (orMore 3) isCreature)))
            (draw 1),
        whenever (does (a isPlayer) (attackWith (counted (orMore 5) isCreature)))
          (sequentially [instruct this_ (dealDamage 3 eachOpponent), gainLife 3]) ],
    power := stat 4, toughness := stat 4 }
