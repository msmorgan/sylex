/-- Land Tax -/
card := {
  name := "Land Tax",
  cost := [white],
  types := [enchantment],
  text := [
    at_ (beginningOfYour upkeep)
      (if_ := exists_
         (compareOver isOpponent (countOf [isLand, controlledBy they]) greater
            (countOf [isLand, youControl])))
      (may
         [
           search yourLibrary (counted (upTo 3) [isLand, hasSupertype basic]),
           reveal (those card),
           putInto (those card) hand,
           shuffle
         ])
  ]
}
