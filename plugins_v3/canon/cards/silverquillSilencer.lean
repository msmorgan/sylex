card silverquillSilencer : Spelled :=
  { name := "Silverquill Silencer", cost := [pip .white, pip .black], types := [.creature],
    subtypes := [human, cleric],
    text :=
      [ static (entersChoosingFrom thisCreature .cardName (nameOfCard (non isLand))),
        whenever (does anOpponent (cast (a (and [isSpell,
          named chosenName]))))
          (sequentially [instruct they (loseLife 3), draw 1]) ],
    power := stat 3, toughness := stat 2 }
