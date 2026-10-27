card ashZealot : Spelled :=
  { name := "Ash Zealot", cost := [pip .red, pip .red], types := [.creature],
    subtypes := [human, warrior],
    text :=
      [ firstStrike,
        haste,
        whenever (does (a isPlayer) (cast (a
          (and [isSpell, did any (cast which (from_ := graveyard)) thisObject]))))
          (instruct thisCreature (dealDamage 3 (that player))) ],
    power := stat 2, toughness := stat 2 }
