card := {
  name := "Conqueror's Pledge",
  cost := [2, white, white, white],
  types := [sorcery],
  text := [
    kicker [6],
    spell
      (replace (create 6 (creatureToken 1 1 [white] [kor, soldier]))
         (doIf (keywordCostPaid kicker this_) (instead (create 12 asThose []))))
  ]
}
