card ivoryTower : Spelled :=
  { name := "Ivory Tower", cost := [generic 1], types := [.artifact],
    text :=
      [ at_ (beginningOfYour .upkeep)
          (sequentially
            [ gainLife (letter .x),
              define .x (minus (countOf (cardIn yourHand)) 4) ]) ] }
