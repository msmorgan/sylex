card := {
  name := "Bonders' Enclave",
  types := [land],
  text := [
    activated tapSymbol (addMana 1 (manaOf [[colorless]])),
    activatedOnlyIf (compound [[3], tapSymbol]) (draw 1)
      (exists_ [isCreature, youControl, hasPower atLeast 4])
  ]
}
