card stormwildCapridor : Spelled :=
  { name := "Stormwild Capridor", cost := [generic 2, pip .white], types := [.creature],
    subtypes := [bird, goat],
    text :=
      [ flying,
        static
          (replace (would any (dealDamage anyAmount thisCreature (some .noncombat)))
            (instead
              (preventDamage allDamage
                (also := putCounters preventedThisWay p1p1Counter thisCreature)))) ],
    power := stat 1, toughness := stat 3 }
