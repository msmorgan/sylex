card := {
  name := "Mutavault",
  types := [land],
  text := [
    activated tapSymbol (addMana 1 (manaOf [[colorless]])),
    activated [1]
      (establishThroughout
         (qualityChange thisLand sets
            (bundle
               { characteristics := { types := [creature], power := 2, toughness := 2 },
                 qualities := [withEveryType creature] }
               (some land)))
         untilEndOfTurn)
  ]
}
