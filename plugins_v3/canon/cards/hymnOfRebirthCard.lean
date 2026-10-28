card := {
  name := "Hymn of Rebirth",
  cost := [3, green, white],
  types := [sorcery],
  text := [
    spell
      (putOntoBattlefieldUnderYourControl (target [isCreature, in_ graveyard]) (from_ := graveyard))
  ]
}
