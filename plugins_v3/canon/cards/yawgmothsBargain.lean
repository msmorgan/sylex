card yawgmothsBargain : Spelled :=
  { name := "Yawgmoth's Bargain", cost := [generic 4, pip .black, pip .black],
    types := [.enchantment],
    text :=
      [ static (partSkip you .drawStep),
        activated (payLife 1) (draw 1) ] }
