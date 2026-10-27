card duneblastCard : Spelled :=
  { name := "Duneblast", cost := [generic 4, pip .white, pip .black, pip .green],
    types := [.sorcery], text := [spell (sequentially [choose (counted (upTo 1) isCreature), destroy
      theRest])] }
