card sonicShrieker : Spelled :=
  { name := "Sonic Shrieker", cost := [generic 2, pip .red, pip .white, pip .black],
    types := [.creature], subtypes := [dragon],
    text :=
      [ flying,
        when (does thisCreature enter)
          (sequentially
            [ instruct it (dealDamage 2 (target isAnyTarget)),
              gainLife 2,
              doIf (was (dealDamage anyAmount (a isPlayer)) thisWay)
                (instruct they (discard (a (cardIn hand)))) ]) ],
    power := stat 4, toughness := stat 4 }
