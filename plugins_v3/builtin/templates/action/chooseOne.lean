/-- "Choose one — <first>; or <second>." -/
template chooseOne (first second : Action) : Action := {
  spelling := "Choose one —\n• <first>\n• <second>",
  body :=
    actionChooseModes (exactly 1) [first, second]
}
