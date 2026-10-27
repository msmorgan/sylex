card coalStoker : Spelled :=
  { name := "Coal Stoker", cost := [generic 3, pip .red], types := [.creature],
    subtypes := [elemental],
    text :=
      [ when (does thisCreature enter)
          (if_ := is it (did you (cast which (from_ := yourHand)) thisObject))
          (addMana 1 (manaOf [[.of .red, .of .red, .of .red]])) ],
    power := stat 3, toughness := stat 3 }
