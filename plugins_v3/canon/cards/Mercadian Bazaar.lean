/-- Mercadian Bazaar -/
card := {
  name := "Mercadian Bazaar",
  types := [land],
  text := [
    entersTapped thisLand,
    activated tapSymbol (putCounters 1 storageCounter thisLand),
    activated (compound [tapSymbol, (removeCounters anyNumber storageCounter thisLand)])
      (addMana removedThisWay (manaOf [[of red]]))
  ]
}
