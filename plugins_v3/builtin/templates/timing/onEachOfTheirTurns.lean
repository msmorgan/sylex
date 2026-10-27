/-- "on each of their turns": each of the turns of the player the deontic subject names, "their"
the singular pronoun resolved distributively ("Each player may play an additional land on each
of their turns"). -/
template onEachOfTheirTurns : Timing := {
  spelling := "on each of their turns",
  body :=
    Semantics.Templates.Timing.timingInPart Semantics.TurnPart.turn
      (Option.some (Semantics.Templates.NounPhrase.nounPhrasePro (Semantics.Reach.word Semantics.NounWord.player) Semantics.Plurality.one))
      (Semantics.Templates.TurnRef.eachTime Option.none)
}
