/-- Coveted Falcon: "Draw a card for each one they gained control of this way" reads the grant
the opponent's "gains control of any number of target permanents you control" recorded
[CR#608.2c,613.1b]. -/
card covetedFalcon : Spelled :=
  { name := "Coveted Falcon", cost := [generic 1, pip .blue, pip .blue],
    types := [.artifact, .creature], subtypes := [bird],
    text :=
      [ flying,
        whenever (does thisCreature attack)
          (gainControl (target (and [isPermanent, youOwn, not (youControl)]))),
        disguise [generic 1, pip .blue],
        when (does any (turnFaceUp thisCreature))
          (sequentially
            [ instruct (target isOpponent)
                (gainControl (targets anyNumber (and [isPermanent, youControl]))),
              draw (forEach (eventTally they (gainControlOf (those permanent)) thisWay .events)
                isPermanent) ]) ],
    power := stat 1, toughness := stat 4 }
