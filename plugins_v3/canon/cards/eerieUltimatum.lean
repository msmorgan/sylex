card eerieUltimatum : Spelled :=
  { name := "Eerie Ultimatum",
    cost := [pip .white, pip .white, pip .black, pip .black, pip .black, pip .green, pip .green],
    types := [.sorcery],
    text :=
      [ spell (returnToBattlefield (withDifferentNames
            (counted anyNumber (and [isPermanentCard, in_ yourGraveyard])))
            (from_ := yourGraveyard)) ] }
