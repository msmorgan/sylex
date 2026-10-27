/-- Land Tax -/
card landTax : Spelled :=
  { name := "Land Tax", cost := [pip .white], types := [.enchantment],
    text :=
      [ at_ (beginningOfYour .upkeep) (if_ := exists_ (compareOver isOpponent (countOf (and [isLand,
        controlledBy they])) .greater
            (countOf (and [isLand, youControl]))))
          (may
            (sequentially
              [ search yourLibrary (counted (upTo 3) (and [isLand, hasSupertype .basic])),
                reveal (those card),
                putInto (those card) hand,
                shuffle ])) ] }
