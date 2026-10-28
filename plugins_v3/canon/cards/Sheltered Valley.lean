card := {
  name := "Sheltered Valley",
  types := [land],
  text := [
    replace (would thisLand enter)
      (instead
         [
           sacrifice (each [isPermanent, isOther, named "Sheltered Valley", youControl]),
           putOntoBattlefield thisLand
         ]),
    at_ (beginningOfYour upkeep) (if_ := compareAmt (countOf [isLand, youControl]) atMost 3)
      (gainLife 1),
    activated tapSymbol (addMana 1 (manaOf [[colorless]]))
  ]
}
