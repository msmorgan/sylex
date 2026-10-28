/-- "during their untap steps": each player's own untap steps, "their" the plural pronoun
resolved to the players the deontic subject names ("Players can't untap …") [CR#502.3]. -/
-- [CR#502.3]
template duringTheirUntapSteps : Timing := {
  spelling := "during their untap steps",
  body :=
    timingInPart Semantics.TurnPart.untapStep
      (some (nounPhrasePro (Semantics.Reach.word Semantics.NounWord.player) Semantics.Plurality.many))
      (eachTime none)
}
