/-- "during their untap steps": each player's own untap steps, "their" the plural pronoun
resolved to the players the deontic subject names ("Players can't untap …") [CR#502.3]. -/
-- [CR#502.3]
template duringTheirUntapSteps : Timing := {
  spelling := "during their untap steps",
  body :=
    timingInPart .untapStep
      (some (nounPhrasePro (.word .player) .many))
      (eachTime none)
}
