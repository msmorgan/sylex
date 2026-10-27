/-- "during your untap step": each of your untap steps [CR#502.3]. -/
-- [CR#502.3]
template duringYourUntapStep : Timing := {
  spelling := "during your untap step",
  body :=
    Semantics.Templates.Timing.timingInPart Semantics.TurnPart.untapStep (Option.some Semantics.Templates.NounPhrase.you)
      (Semantics.Templates.TurnRef.eachTime Option.none)
}
