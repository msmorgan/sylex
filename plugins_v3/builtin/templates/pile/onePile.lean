/-- "one pile": one of the piles just made. -/
template onePile : NounPhrase := {
  spelling := "one pile",
  body :=
    Semantics.Templates.NounPhrase.pileOf (Semantics.Templates.SliceCount.sliceCountCounted (Semantics.Templates.Quantity.exactly 1)) Option.none
}
