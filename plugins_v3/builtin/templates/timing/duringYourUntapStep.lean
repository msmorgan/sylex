/-- "during your untap step": each of your untap steps [CR#502.3]. -/
-- [CR#502.3]
template duringYourUntapStep : Timing := {
  spelling := "during your untap step",
  body :=
    inPart .untapStep (some you)
      (eachTime none)
}
