/-- Unblinking Observer -/
card unblinkingObserver : Spelled :=
  { name := "Unblinking Observer", cost := [generic 1, pip .blue], types := [.creature],
    subtypes := [homunculus],
    text :=
      [ activated tapSymbol
          (addMana 1 (manaOf [[.of .blue]])
            [spendOnly [toPay (.ofKeyword disturb), toCast isInstantOrSorcery]]) ],
    power := stat 2, toughness := stat 1 }
