/-- "the pile of <player>'s choice" -/
template pileOfChoice (player : NounPhrase) : NounPhrase := {
  spelling := "the pile of <player>'s choice",
  body :=
    Semantics.Templates.NounPhrase.pileOf (Semantics.Templates.SliceCount.sliceCountCounted (Semantics.Templates.Quantity.exactly 1)) (Option.some player)
}
