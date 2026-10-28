card := {
  name := "Mishra's Factory",
  types := [land],
  text := [
    activated tapSymbol (addMana 1 (manaOf [[colorless]])),
    activated [1]
      (establishThroughout
         (qualityChange thisLand sets
            (bundle
               {
                 types := [artifact, creature],
                 subtypes := [assemblyWorker],
                 power := 2,
                 toughness := 2
               }
               (some land)))
         untilEndOfTurn),
    activated tapSymbol (get (target [isCreature, assemblyWorker]) (up 1) (up 1) untilEndOfTurn)
  ]
}
