card := {
  name := "Mutavault",
  types := [land],
  text := [
    activated tapSymbol (addMana 1 (manaOf [[colorless]])),
    activated [1]
      (establishThroughout
         (qualityChange thisLand sets
            (bundle ⟨{ types := [creature], power := 2, toughness := 2 }, [withEveryType creature]⟩
               (some land)))
         untilEndOfTurn)
  ]
}
