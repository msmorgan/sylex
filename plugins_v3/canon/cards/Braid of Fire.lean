/-- Braid of Fire -/
card := {
  name := "Braid of Fire",
  cost := [1, red],
  types := [enchantment],
  text := [cumulativeUpkeep (addMana 1 (manaOf [[of red]]))]
}
