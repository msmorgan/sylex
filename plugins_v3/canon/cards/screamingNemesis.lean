card screamingNemesis : Spelled :=
  { name := "Screaming Nemesis", cost := [generic 2, pip .red], types := [.creature],
    subtypes := [spirit],
    text :=
      [ haste,
        whenever (does any (dealDamage (to := thisCreature)))
          (sequentially
            [ instruct it (dealDamage thatMuch (target isAnyOtherTarget)),
              doIf (was (dealDamage anyAmount (a isPlayer)) thisWay)
                (establishThroughout (cant they (gainLife anyAmount)) restOfGame) ]) ],
    power := stat 3, toughness := stat 3 }
