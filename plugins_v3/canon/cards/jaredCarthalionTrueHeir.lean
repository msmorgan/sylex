/-- Jared Carthalion, True Heir: "You can't become the monarch this turn" is a prohibition over
`gainDesignation`, an event head [CR#101.2,725.1]. -/
card jaredCarthalionTrueHeir : Spelled :=
  { name := "Jared Carthalion, True Heir", cost := [pip .red, pip .green, pip .white],
    supertypes := [.legendary], types := [.creature], subtypes := [human, warrior],
    text :=
      [ when (does thisCreature enter)
          (sequentially
            [ gainDesignation (target isOpponent) monarch,
              establishThroughout (cant you (gainDesignation you monarch)) thisTurn ]),
        static
          (onlyWhile
            (replace (would any (dealDamage anyAmount thisCreature))
              (instead
                (preventDamage allDamage
                  (also := putCounters preventedThisWay p1p1Counter thisCreature))))
            (is you (hasDesignation monarch))) ],
    power := stat 3, toughness := stat 3 }
