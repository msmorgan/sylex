/-- Braid of Fire -/
card braidOfFire : Spelled :=
  { name := "Braid of Fire", cost := [generic 1, pip .red], types := [.enchantment],
    text := [cumulativeUpkeep (addMana 1 (manaOf [[.of .red]]))] }
