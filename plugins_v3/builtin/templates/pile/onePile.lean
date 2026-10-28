/-- "one pile": one of the piles just made. -/
template onePile : NounPhrase := {
  spelling := "one pile",
  body :=
    pileOf (sliceCountCounted (exactly 1)) none
}
