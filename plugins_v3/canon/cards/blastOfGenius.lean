card blastOfGenius : Spelled :=
  { name := "Blast of Genius", cost := [generic 4, pip .blue, pip .red], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ choose (target isAnyTarget),
            draw 3,
            discard (a (cardIn yourHand)),
            instruct this_
              (dealDamage (statOf (.stat .manaValue) (the (and [was (discard which), isCard])))
                thatJoin) ]) ] }
