card wickedAkuba : Spelled :=
  { name := "Wicked Akuba", cost := [pip .black, pip .black], types := [.creature],
    subtypes := [spirit],
    text :=
      [ activated [pip .black]
          (instruct
             (target
                (and
                   [ isPlayer,
                     did thisCreature (dealDamage anyAmount which) (within soFarThisTurn) ]))
            (loseLife 1)) ],
    power := stat 2, toughness := stat 2 }
