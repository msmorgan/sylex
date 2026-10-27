card assembleThePlayers : Spelled :=
  { name := "Assemble the Players", cost := [generic 1, pip .white], types := [.enchantment],
    text :=
      [ static (visibility .lookAt you topOfLibrary),
        static castSmallCreatureFromTopOnceEachTurn ] }
