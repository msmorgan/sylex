card hymnOfRebirthCard : Spelled :=
  { name := "Hymn of Rebirth", cost := [generic 3, pip .green, pip .white],
    types := [.sorcery], text := [spell (putOntoBattlefieldUnderYourControl (target (and
      [isCreature, in_ graveyard]))
          (from_ := graveyard))] }
