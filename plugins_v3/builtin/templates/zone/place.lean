/-- "on top of <pile>", "on the bottom of <pile>": a destination at one end of a pile. -/
template place (end_ : PileEnd) (pile : Location) : Location := {
  spelling := "the <end_> of <pile>",
  body := fromEnd end_ (nth 1) pile
}
