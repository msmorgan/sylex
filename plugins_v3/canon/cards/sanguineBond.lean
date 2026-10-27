card sanguineBond : Spelled :=
  { name := "Sanguine Bond", cost := [generic 3, pip .black, pip .black],
    types := [.enchantment],
    text := [whenever (does you (gainLife anyAmount))
               (instruct (target isOpponent) (loseLife thatMuch))]
        }
