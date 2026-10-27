card soulScarMage : Spelled :=
  { name := "Soul-Scar Mage", cost := [pip .red], types := [.creature],
    subtypes := [human, wizard],
    text :=
      [ prowess,
        static
          (replace
            (would (a (and [isSource, youControl]))
              (dealDamage anyAmount (a (and [isCreature, anOpponentControls])) (some .noncombat)))
            (instead (putCounters thatMuch m1m1Counter (that creature)))) ],
    power := stat 1, toughness := stat 2 }
