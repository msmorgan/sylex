card := {
  name := "Culling Scales",
  cost := [3],
  types := [artifact],
  text := [
    at_ (beginningOfYour upkeep)
      (destroy
         (target
            [isPermanent, non isLand, superlative min (stat manaValue) [isPermanent, non isLand]]))
  ]
}
