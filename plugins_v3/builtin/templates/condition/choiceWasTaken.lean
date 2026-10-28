/-- "if <subject>'s <which> was taken": the choice the Link Source offered was taken
[CR#702.104b]. -/
-- [CR#702.104b]
template choiceWasTaken (which : LinkSource) (subject : NounPhrase) : Condition := {
  spelling := "if <subject>'s <which> was taken",
  body :=
    compareAmt (paid (linkedFacetTaken which) subject)
      Semantics.Comparator.atLeast 1
}
