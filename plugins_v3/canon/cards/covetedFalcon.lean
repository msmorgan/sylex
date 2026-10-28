/-- Coveted Falcon: "Draw a card for each one they gained control of this way" reads the grant
the opponent's "gains control of any number of target permanents you control" recorded
[CR#608.2c,613.1b]. -/
card := {
  name := "Coveted Falcon",
  cost := [1, blue, blue],
  types := [artifact, creature],
  subtypes := [bird],
  text := [
    flying,
    whenever (does thisCreature attack)
      (gainControl (target [isPermanent, youOwn, not (youControl)])),
    disguise [1, blue],
    when (does any (turnFaceUp thisCreature))
      [
        instruct (target isOpponent) (gainControl (targets anyNumber [isPermanent, youControl])),
        draw
          (forEach (eventTally they (gainControlOf (those permanent)) thisWay events) isPermanent)
      ]
  ],
  power := 1,
  toughness := 4
}
