card seethingSong : Spelled :=
  { name := "Seething Song", cost := [generic 2, pip .red], types := [.instant],
    text :=
      [ spell (addMana 1 (manaOf [[.of .red, .of .red, .of .red, .of .red, .of .red]])) ] }
