/-- Death or Glory -/
card := {
  name := "Death or Glory",
  cost := [4, white],
  types := [sorcery],
  text := [
    spell
      [
        separateIntoPiles (allOf [isCreature, in_ yourGraveyard]) 2,
        exile (pileOfChoice anOpponent),
        returnToBattlefield theOther
      ]
  ]
}
